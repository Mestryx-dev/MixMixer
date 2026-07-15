use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

pub struct AudioMetrics {
    voice_buffer_samples: AtomicUsize,
    voice_buffer_capacity: AtomicUsize,
    streams_active: AtomicUsize,
    sample_rate: AtomicU32,
    buffer_frames: AtomicU32,
    routing_live: AtomicBool,
    reconnect_pending: AtomicBool,
}

impl AudioMetrics {
    pub fn new() -> Self {
        Self {
            voice_buffer_samples: AtomicUsize::new(0),
            voice_buffer_capacity: AtomicUsize::new(1),
            streams_active: AtomicUsize::new(0),
            sample_rate: AtomicU32::new(48_000),
            buffer_frames: AtomicU32::new(128),
            routing_live: AtomicBool::new(false),
            reconnect_pending: AtomicBool::new(false),
        }
    }

    pub fn set_voice_buffer(&self, samples: usize, capacity: usize) {
        self.voice_buffer_samples.store(samples, Ordering::Relaxed);
        self.voice_buffer_capacity
            .store(capacity.max(1), Ordering::Relaxed);
    }

    pub fn set_streams(&self, count: usize) {
        self.streams_active.store(count, Ordering::Relaxed);
    }

    pub fn set_config(&self, sample_rate: u32, buffer_frames: u32) {
        self.sample_rate
            .store(sample_rate.max(1), Ordering::Relaxed);
        self.buffer_frames
            .store(buffer_frames.max(1), Ordering::Relaxed);
    }

    pub fn set_routing_live(&self, live: bool) {
        self.routing_live.store(live, Ordering::Relaxed);
    }

    pub fn set_reconnect_pending(&self, pending: bool) {
        self.reconnect_pending.store(pending, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricsSnapshot {
        let capacity = self.voice_buffer_capacity.load(Ordering::Relaxed);
        let samples = self.voice_buffer_samples.load(Ordering::Relaxed);
        let sample_rate = self.sample_rate.load(Ordering::Relaxed);
        let buffer_frames = self.buffer_frames.load(Ordering::Relaxed);

        MetricsSnapshot {
            estimated_latency_ms: estimated_latency_ms(sample_rate, buffer_frames),
            voice_buffer_pct: ((samples as f32 / capacity as f32) * 100.0).min(100.0),
            streams_active: self.streams_active.load(Ordering::Relaxed),
            routing_live: self.routing_live.load(Ordering::Relaxed),
            reconnect_pending: self.reconnect_pending.load(Ordering::Relaxed),
        }
    }
}

pub struct MetricsSnapshot {
    pub estimated_latency_ms: f32,
    pub voice_buffer_pct: f32,
    pub streams_active: usize,
    pub routing_live: bool,
    pub reconnect_pending: bool,
}

fn estimated_latency_ms(sample_rate: u32, buffer_frames: u32) -> f32 {
    // Capture buffer + output buffer (approximation WASAPI shared mode).
    (buffer_frames as f32 * 2_000.0) / sample_rate as f32
}
