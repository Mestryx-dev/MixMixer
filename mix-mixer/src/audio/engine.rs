use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{
    BufferSize, Sample, SampleFormat, SampleRate, Stream, StreamConfig, StreamError,
    SupportedBufferSize,
};
use crossbeam_channel::Receiver;
use tracing::{info, warn};

use crate::audio::metrics::AudioMetrics;
use crate::audio::mixer::{apply_voice_gain, MixControls};
use crate::audio::resampler::{from_stereo_interleaved, to_stereo_interleaved, StereoResampler};
use crate::audio::ring_buffer::SpscRingBuffer;
use crate::config::Config;
use crate::devices::{default_host, find_input_device, find_output_device};
use crate::error::{Error, Result};

const RING_CAPACITY_SAMPLES: usize = 4096 * 2;
const RECONNECT_INITIAL_MS: u64 = 400;
const RECONNECT_MAX_MS: u64 = 5000;
const RELEASE_SETTLE_MS: u64 = 300;

struct ReconnectState {
    pending: bool,
    next_at: Instant,
    delay_ms: u64,
}

impl ReconnectState {
    fn idle() -> Self {
        Self {
            pending: false,
            next_at: Instant::now(),
            delay_ms: RECONNECT_INITIAL_MS,
        }
    }

    fn schedule_immediate(delay_ms: u64) -> Self {
        Self {
            pending: true,
            next_at: Instant::now() + Duration::from_millis(delay_ms),
            delay_ms,
        }
    }

    fn schedule_fault(&mut self) {
        self.pending = true;
        self.delay_ms = RECONNECT_INITIAL_MS;
        self.next_at = Instant::now() + Duration::from_millis(self.delay_ms);
    }

    fn backoff(&mut self) {
        self.delay_ms = (self.delay_ms * 2).min(RECONNECT_MAX_MS);
        self.next_at = Instant::now() + Duration::from_millis(self.delay_ms);
    }
}

pub enum AudioCommand {
    ToggleMonitor,
    ReloadConfig(Box<Config>),
    RestartWithConfig(Box<Config>),
    SetRoutingEnabled(bool),
    Shutdown,
}

pub struct AudioEngine {
    config: Config,
    controls: Arc<MixControls>,
    voice_rb: Arc<SpscRingBuffer>,
    monitor_rb: Arc<SpscRingBuffer>,
    stream_fault: Arc<AtomicBool>,
    metrics: Arc<AudioMetrics>,
    reconnect: ReconnectState,
    _streams: Vec<Stream>,
}

impl AudioEngine {
    pub fn new(config: Config, metrics: Arc<AudioMetrics>) -> Result<Self> {
        let controls = Arc::new(MixControls::from_gains(
            &config.gains,
            config.monitor.enabled,
        ));
        let voice_rb = Arc::new(SpscRingBuffer::new(RING_CAPACITY_SAMPLES));
        let monitor_rb = Arc::new(SpscRingBuffer::new(RING_CAPACITY_SAMPLES));

        let stream_fault = Arc::new(AtomicBool::new(false));

        let mut engine = Self {
            config,
            controls,
            voice_rb,
            monitor_rb,
            stream_fault,
            metrics,
            reconnect: ReconnectState::idle(),
            _streams: Vec::new(),
        };

        engine.sync_metrics();

        if engine.config.enabled {
            if let Err(err) = engine.start_streams() {
                warn!(%err, "initial stream start failed — will retry");
                engine.reconnect = ReconnectState::schedule_immediate(RECONNECT_INITIAL_MS);
            }
        } else {
            info!("routing disabled at startup");
        }

        Ok(engine)
    }

    pub fn run(&mut self, cmd_rx: Receiver<AudioCommand>) -> Result<()> {
        loop {
            match cmd_rx.recv_timeout(Duration::from_millis(50)) {
                Ok(AudioCommand::ToggleMonitor) => {
                    let enabled = self.controls.toggle_monitor();
                    info!(enabled, "monitor toggled");
                }
                Ok(AudioCommand::ReloadConfig(cfg)) => {
                    self.apply_config(*cfg, false);
                }
                Ok(AudioCommand::RestartWithConfig(cfg)) => {
                    self.apply_config(*cfg, true);
                }
                Ok(AudioCommand::SetRoutingEnabled(enabled)) => {
                    self.config.enabled = enabled;
                    if enabled {
                        self.reconnect.schedule_fault();
                    } else {
                        self.reconnect = ReconnectState::idle();
                        self.release_streams();
                        info!("routing disabled");
                    }
                }
                Ok(AudioCommand::Shutdown) => {
                    info!("audio shutdown");
                    break;
                }
                Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
                Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
            }
            self.poll_stream_health();
        }
        Ok(())
    }

    fn poll_stream_health(&mut self) {
        if !self.config.enabled {
            self.reconnect = ReconnectState::idle();
            self.sync_metrics();
            return;
        }

        if self.stream_fault.swap(false, Ordering::AcqRel) {
            warn!("audio device fault — scheduling reconnect");
            self.release_streams();
            self.reconnect.schedule_fault();
        } else if self._streams.is_empty() && !self.reconnect.pending {
            self.reconnect.schedule_fault();
        }

        if !self.reconnect.pending || Instant::now() < self.reconnect.next_at {
            self.sync_metrics();
            return;
        }

        self.voice_rb.clear();
        self.monitor_rb.clear();
        match self.start_streams() {
            Ok(()) => {
                info!("audio reconnected");
                self.reconnect = ReconnectState::idle();
            }
            Err(err) => {
                warn!(%err, delay_ms = self.reconnect.delay_ms, "reconnect failed — retrying");
                self.release_streams();
                self.reconnect.backoff();
            }
        }
        self.sync_metrics();
    }

    fn sync_metrics(&self) {
        self.metrics
            .set_voice_buffer(self.voice_rb.available(), RING_CAPACITY_SAMPLES);
        self.metrics.set_streams(self._streams.len());
        self.metrics
            .set_config(self.config.sample_rate, self.config.buffer_frames);
        self.metrics
            .set_routing_live(self.config.enabled && !self._streams.is_empty());
        self.metrics.set_reconnect_pending(self.reconnect.pending);
    }

    fn apply_config(&mut self, config: Config, restart_streams: bool) {
        self.config = config;
        self.controls.apply_gains(&self.config.gains);
        self.controls
            .set_monitor_enabled(self.config.monitor.enabled);

        if !self.config.enabled {
            self.release_streams();
            info!("config updated — routing disabled");
            return;
        }

        if restart_streams || self._streams.is_empty() {
            if let Err(err) = self.restart_streams(self.config.clone()) {
                warn!(%err, "audio restart failed");
            }
        } else {
            info!("config reloaded");
        }
    }

    fn start_streams(&mut self) -> Result<()> {
        let streams = self.build_streams()?;
        for stream in &streams {
            stream
                .play()
                .map_err(|e| Error::audio(format!("stream play: {e}")))?;
        }
        self._streams = streams;
        info!(streams = self._streams.len(), "audio streams started");
        self.sync_metrics();
        Ok(())
    }

    fn restart_streams(&mut self, config: Config) -> Result<()> {
        let old_config = self.config.clone();
        self.release_streams();

        self.config = config;
        self.controls.apply_gains(&self.config.gains);
        self.controls
            .set_monitor_enabled(self.config.monitor.enabled);
        self.voice_rb.clear();
        self.monitor_rb.clear();

        const MAX_ATTEMPTS: u32 = 10;
        for attempt in 0..MAX_ATTEMPTS {
            match self.build_streams() {
                Ok(streams) => {
                    for stream in &streams {
                        stream
                            .play()
                            .map_err(|e| Error::audio(format!("stream play: {e}")))?;
                    }
                    self._streams = streams;
                    info!(
                        streams = self._streams.len(),
                        attempt = attempt + 1,
                        "audio streams restarted"
                    );
                    return Ok(());
                }
                Err(err) if attempt + 1 < MAX_ATTEMPTS => {
                    warn!(%err, attempt = attempt + 1, "stream rebuild failed — retrying");
                    self.release_streams();
                    std::thread::sleep(Duration::from_millis(50 * (attempt as u64 + 1)));
                }
                Err(err) => {
                    warn!(%err, "stream rebuild failed — restoring previous audio");
                    self.release_streams();
                    self.config = old_config.clone();
                    self.controls.apply_gains(&self.config.gains);
                    self.controls
                        .set_monitor_enabled(self.config.monitor.enabled);
                    self.voice_rb.clear();
                    self.monitor_rb.clear();
                    return self.restore_streams(&old_config);
                }
            }
        }

        Err(Error::audio("stream rebuild exhausted retries"))
    }

    fn release_streams(&mut self) {
        let old_streams = std::mem::take(&mut self._streams);
        for stream in &old_streams {
            let _ = stream.pause();
        }
        drop(old_streams);
        std::thread::sleep(Duration::from_millis(RELEASE_SETTLE_MS));
        self.sync_metrics();
    }

    fn restore_streams(&mut self, config: &Config) -> Result<()> {
        self.config = config.clone();
        self.controls.apply_gains(&self.config.gains);
        self.controls
            .set_monitor_enabled(self.config.monitor.enabled);
        if !self.config.enabled {
            info!("routing disabled — skip restore");
            return Ok(());
        }
        match self.build_streams() {
            Ok(streams) => {
                for stream in &streams {
                    stream
                        .play()
                        .map_err(|e| Error::audio(format!("stream play: {e}")))?;
                }
                self._streams = streams;
                info!(streams = self._streams.len(), "previous audio restored");
                Ok(())
            }
            Err(err) => {
                warn!(%err, "failed to restore previous audio");
                Err(err)
            }
        }
    }

    fn build_streams(&self) -> Result<Vec<Stream>> {
        let stream_fault = Arc::clone(&self.stream_fault);
        let host = default_host();
        let voice_dev = find_input_device(&host, &self.config.devices.voice_input)?;
        let out_dev = find_output_device(&host, &self.config.devices.virtual_mic_output)?;

        let voice_name = voice_dev.name().unwrap_or_default();
        let out_name = out_dev.name().unwrap_or_default();
        info!(%voice_name, %out_name, "micro → VAC");

        let voice_cfg = pick_stream_config(
            &voice_dev,
            true,
            self.config.sample_rate,
            2,
            self.config.buffer_frames,
        )?;
        let out_cfg = pick_stream_config(
            &out_dev,
            false,
            self.config.sample_rate,
            2,
            self.config.buffer_frames,
        )?;

        let mut streams = Vec::new();

        streams.push(open_input_stream(
            &voice_dev,
            self.config.sample_rate,
            2,
            self.config.buffer_frames,
            &voice_cfg,
            Arc::clone(&self.voice_rb),
            Arc::clone(&stream_fault),
        )?);

        let controls = Arc::clone(&self.controls);
        let voice_rb = Arc::clone(&self.voice_rb);
        let monitor_rb = Arc::clone(&self.monitor_rb);
        let out_channels = out_cfg.0.channels;

        streams.push(build_output_stream(
            &out_dev,
            &out_cfg,
            Arc::clone(&stream_fault),
            move |data: &mut [f32]| {
                render_voice(data, out_channels, &controls, &voice_rb, &monitor_rb);
            },
        )?);

        if self.config.monitor.enabled {
            if let Ok(mon_dev) = find_output_device(&host, &self.config.devices.monitor_output) {
                let mon_name = mon_dev.name().unwrap_or_default();
                info!(%mon_name, "monitor output");
                let mon_cfg = pick_stream_config(
                    &mon_dev,
                    false,
                    self.config.sample_rate,
                    2,
                    self.config.buffer_frames,
                )?;
                let mon_channels = mon_cfg.0.channels;
                let monitor_rb = Arc::clone(&self.monitor_rb);
                let controls = Arc::clone(&self.controls);

                streams.push(build_output_stream(
                    &mon_dev,
                    &mon_cfg,
                    Arc::clone(&stream_fault),
                    move |data: &mut [f32]| {
                        render_monitor(data, mon_channels, &controls, &monitor_rb);
                    },
                )?);
            } else {
                warn!(
                    query = %self.config.devices.monitor_output,
                    "monitor output not found"
                );
            }
        }

        Ok(streams)
    }
}

fn pick_stream_config(
    device: &cpal::Device,
    is_input: bool,
    target_rate: u32,
    target_channels: u16,
    buffer_frames: u32,
) -> Result<(StreamConfig, SampleFormat)> {
    let default = if is_input {
        device
            .default_input_config()
            .map_err(|e| Error::device(format!("default input config: {e}")))?
    } else {
        device
            .default_output_config()
            .map_err(|e| Error::device(format!("default output config: {e}")))?
    };

    let format = default.sample_format();
    let mut config = default.config();
    config.channels = target_channels.max(default.channels());
    config.sample_rate = SampleRate(target_rate);
    config.buffer_size = BufferSize::Fixed(buffer_frames);

    if let SupportedBufferSize::Range { min, .. } = default.buffer_size() {
        if let BufferSize::Fixed(f) = config.buffer_size {
            if f < *min {
                config.buffer_size = BufferSize::Fixed(*min);
            }
        }
    }

    Ok((config, format))
}

fn pick_native_stream_config(
    device: &cpal::Device,
    is_input: bool,
    target_channels: u16,
    buffer_frames: u32,
) -> Result<(StreamConfig, SampleFormat)> {
    let default = if is_input {
        device
            .default_input_config()
            .map_err(|e| Error::device(format!("default input config: {e}")))?
    } else {
        device
            .default_output_config()
            .map_err(|e| Error::device(format!("default output config: {e}")))?
    };

    let format = default.sample_format();
    let mut config = default.config();
    config.channels = target_channels.max(default.channels());
    config.buffer_size = BufferSize::Fixed(buffer_frames);

    if let SupportedBufferSize::Range { min, .. } = default.buffer_size() {
        if let BufferSize::Fixed(f) = config.buffer_size {
            if f < *min {
                config.buffer_size = BufferSize::Fixed(*min);
            }
        }
    }

    Ok((config, format))
}

fn open_input_stream(
    device: &cpal::Device,
    master_rate: u32,
    target_channels: u16,
    buffer_frames: u32,
    preferred: &(StreamConfig, SampleFormat),
    rb: Arc<SpscRingBuffer>,
    stream_fault: Arc<AtomicBool>,
) -> Result<Stream> {
    match build_input_stream(
        device,
        preferred,
        Arc::clone(&rb),
        master_rate,
        Arc::clone(&stream_fault),
    ) {
        Ok(stream) => Ok(stream),
        Err(err) => {
            warn!(
                %err,
                "preferred input config failed — falling back to native rate + resampler"
            );
            let native = pick_native_stream_config(device, true, target_channels, buffer_frames)?;
            build_input_stream(device, &native, rb, master_rate, stream_fault)
        }
    }
}

fn build_input_stream(
    device: &cpal::Device,
    (config, format): &(StreamConfig, SampleFormat),
    rb: Arc<SpscRingBuffer>,
    master_rate: u32,
    stream_fault: Arc<AtomicBool>,
) -> Result<Stream> {
    let channels = config.channels;
    let stream_rate = config.sample_rate.0;
    let resampler: Arc<Mutex<Option<StereoResampler>>> = if stream_rate != master_rate {
        info!(stream_rate, master_rate, "input resampling enabled");
        Arc::new(Mutex::new(Some(StereoResampler::new(
            stream_rate,
            master_rate,
            256,
        )?)))
    } else {
        Arc::new(Mutex::new(None))
    };

    let err_fn = make_stream_error_handler(stream_fault);

    let stream = match format {
        SampleFormat::F32 => {
            let rb = Arc::clone(&rb);
            let resampler = Arc::clone(&resampler);
            device
                .build_input_stream(
                    config,
                    move |data: &[f32], _| {
                        let stereo = to_stereo_interleaved(data, channels);
                        push_resampled(&rb, &resampler, stereo);
                    },
                    err_fn,
                    None,
                )
                .map_err(map_stream_err)?
        }
        SampleFormat::I16 => {
            let rb = Arc::clone(&rb);
            let resampler = Arc::clone(&resampler);
            device
                .build_input_stream(
                    config,
                    move |data: &[i16], _| {
                        let f32s: Vec<f32> = data.iter().map(|&s| Sample::to_sample(s)).collect();
                        let stereo = to_stereo_interleaved(&f32s, channels);
                        push_resampled(&rb, &resampler, stereo);
                    },
                    err_fn,
                    None,
                )
                .map_err(map_stream_err)?
        }
        SampleFormat::I32 => {
            let rb = Arc::clone(&rb);
            let resampler = Arc::clone(&resampler);
            device
                .build_input_stream(
                    config,
                    move |data: &[i32], _| {
                        let f32s: Vec<f32> = data.iter().map(|&s| Sample::to_sample(s)).collect();
                        let stereo = to_stereo_interleaved(&f32s, channels);
                        push_resampled(&rb, &resampler, stereo);
                    },
                    err_fn,
                    None,
                )
                .map_err(map_stream_err)?
        }
        SampleFormat::I8 => {
            let rb = Arc::clone(&rb);
            let resampler = Arc::clone(&resampler);
            device
                .build_input_stream(
                    config,
                    move |data: &[i8], _| {
                        let f32s: Vec<f32> = data.iter().map(|&s| Sample::to_sample(s)).collect();
                        let stereo = to_stereo_interleaved(&f32s, channels);
                        push_resampled(&rb, &resampler, stereo);
                    },
                    err_fn,
                    None,
                )
                .map_err(map_stream_err)?
        }
        SampleFormat::U8 => {
            let rb = Arc::clone(&rb);
            let resampler = Arc::clone(&resampler);
            device
                .build_input_stream(
                    config,
                    move |data: &[u8], _| {
                        let f32s: Vec<f32> = data.iter().map(|&s| Sample::to_sample(s)).collect();
                        let stereo = to_stereo_interleaved(&f32s, channels);
                        push_resampled(&rb, &resampler, stereo);
                    },
                    err_fn,
                    None,
                )
                .map_err(map_stream_err)?
        }
        other => {
            return Err(Error::audio(format!(
                "unsupported input sample format: {other:?}"
            )));
        }
    };

    Ok(stream)
}

fn push_resampled(
    rb: &SpscRingBuffer,
    resampler: &Mutex<Option<StereoResampler>>,
    stereo: Vec<f32>,
) {
    let samples = if let Ok(mut guard) = resampler.lock() {
        if let Some(ref mut rs) = *guard {
            rs.process_interleaved(&stereo).unwrap_or(stereo)
        } else {
            stereo
        }
    } else {
        stereo
    };
    rb.push(&samples);
}

fn build_output_stream<F>(
    device: &cpal::Device,
    (config, format): &(StreamConfig, SampleFormat),
    stream_fault: Arc<AtomicBool>,
    mut render: F,
) -> Result<Stream>
where
    F: FnMut(&mut [f32]) + Send + 'static,
{
    let channels = config.channels;
    let err_fn = make_stream_error_handler(stream_fault);

    let stream = match format {
        SampleFormat::F32 => device
            .build_output_stream(
                config,
                move |data: &mut [f32], _| {
                    let mut scratch = vec![0.0f32; data.len()];
                    render(&mut scratch);
                    data.copy_from_slice(&scratch);
                },
                err_fn,
                None,
            )
            .map_err(map_stream_err)?,
        SampleFormat::I16 => device
            .build_output_stream(
                config,
                move |data: &mut [i16], _| {
                    let mut scratch = vec![0.0f32; data.len()];
                    render(&mut scratch);
                    let mapped: Vec<f32> = from_stereo_interleaved(&scratch, channels);
                    for (out, sample) in data.iter_mut().zip(mapped.iter()) {
                        *out = Sample::from_sample(*sample);
                    }
                },
                err_fn,
                None,
            )
            .map_err(map_stream_err)?,
        SampleFormat::I32 => device
            .build_output_stream(
                config,
                move |data: &mut [i32], _| {
                    let mut scratch = vec![0.0f32; data.len()];
                    render(&mut scratch);
                    let mapped: Vec<f32> = from_stereo_interleaved(&scratch, channels);
                    for (out, sample) in data.iter_mut().zip(mapped.iter()) {
                        *out = Sample::from_sample(*sample);
                    }
                },
                err_fn,
                None,
            )
            .map_err(map_stream_err)?,
        other => {
            return Err(Error::audio(format!(
                "unsupported output sample format: {other:?}"
            )));
        }
    };

    Ok(stream)
}

fn render_voice(
    out: &mut [f32],
    out_channels: u16,
    controls: &MixControls,
    voice_rb: &SpscRingBuffer,
    monitor_rb: &SpscRingBuffer,
) {
    let frames = out.len() / out_channels as usize;
    let stereo_len = frames * 2;
    let mut voice = vec![0.0f32; stereo_len];
    let mut stereo_out = vec![0.0f32; stereo_len];

    voice_rb.pop_into(&mut voice);
    apply_voice_gain(controls, &mut voice, &mut stereo_out);

    if controls.monitor_enabled() {
        monitor_rb.push(&stereo_out);
    }

    if out_channels == 2 {
        out.copy_from_slice(&stereo_out);
    } else {
        let mapped = from_stereo_interleaved(&stereo_out, out_channels);
        out.copy_from_slice(&mapped);
    }
}

fn render_monitor(
    out: &mut [f32],
    out_channels: u16,
    controls: &MixControls,
    monitor_rb: &SpscRingBuffer,
) {
    if !controls.monitor_enabled() {
        out.fill(0.0);
        return;
    }

    let frames = out.len() / out_channels as usize;
    let stereo_len = frames * 2;
    let mut stereo_buf = vec![0.0f32; stereo_len];
    monitor_rb.pop_into(&mut stereo_buf);

    if out_channels == 2 {
        out.copy_from_slice(&stereo_buf);
    } else {
        let mapped = from_stereo_interleaved(&stereo_buf, out_channels);
        out.copy_from_slice(&mapped);
    }
}

fn make_stream_error_handler(
    stream_fault: Arc<AtomicBool>,
) -> impl FnMut(StreamError) + Send + 'static {
    move |err| {
        warn!(%err, "audio stream error");
        stream_fault.store(true, Ordering::Release);
    }
}

fn map_stream_err(err: cpal::BuildStreamError) -> Error {
    Error::audio(format!("build stream: {err}"))
}
