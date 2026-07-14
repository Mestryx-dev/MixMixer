use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::config::Gains;

#[derive(Debug)]
pub struct MixControls {
    voice_gain: AtomicU32,
    master_gain: AtomicU32,
    monitor_enabled: AtomicBool,
}

impl MixControls {
    pub fn from_gains(gains: &Gains, monitor_enabled: bool) -> Self {
        Self {
            voice_gain: AtomicU32::new(gains.voice.to_bits()),
            master_gain: AtomicU32::new(gains.master.to_bits()),
            monitor_enabled: AtomicBool::new(monitor_enabled),
        }
    }

    pub fn apply_gains(&self, gains: &Gains) {
        self.voice_gain.store(gains.voice.to_bits(), Ordering::Relaxed);
        self.master_gain.store(gains.master.to_bits(), Ordering::Relaxed);
    }

    pub fn toggle_monitor(&self) -> bool {
        let prev = self.monitor_enabled.load(Ordering::Relaxed);
        self.monitor_enabled.store(!prev, Ordering::Relaxed);
        !prev
    }

    pub fn voice_gain(&self) -> f32 {
        f32::from_bits(self.voice_gain.load(Ordering::Relaxed))
    }

    pub fn master_gain(&self) -> f32 {
        f32::from_bits(self.master_gain.load(Ordering::Relaxed))
    }

    pub fn monitor_enabled(&self) -> bool {
        self.monitor_enabled.load(Ordering::Relaxed)
    }

    pub fn set_monitor_enabled(&self, enabled: bool) {
        self.monitor_enabled.store(enabled, Ordering::Relaxed);
    }
}

pub fn apply_voice_gain(controls: &MixControls, voice: &mut [f32], out: &mut [f32]) {
    debug_assert_eq!(voice.len(), out.len());
    let voice_gain = controls.voice_gain();
    let master = controls.master_gain();

    for i in 0..out.len() {
        out[i] = soft_clip(voice[i] * voice_gain * master);
    }
}

#[inline]
fn soft_clip(x: f32) -> f32 {
    x.clamp(-1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_voice_gain_scales_samples() {
        let controls = MixControls::from_gains(
            &Gains {
                voice: 0.5,
                master: 1.0,
            },
            false,
        );
        let mut voice = [1.0f32, -1.0];
        let mut out = [0.0; 2];
        apply_voice_gain(&controls, &mut voice, &mut out);
        assert_eq!(out, [0.5, -0.5]);
    }
}
