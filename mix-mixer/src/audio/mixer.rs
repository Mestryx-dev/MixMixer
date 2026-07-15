use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use crate::config::Monitor;

#[derive(Debug)]
pub struct MixControls {
    monitor_enabled: AtomicBool,
    monitor_volume: AtomicU32,
}

impl MixControls {
    pub fn from_monitor(monitor: &Monitor) -> Self {
        Self {
            monitor_enabled: AtomicBool::new(monitor.enabled),
            monitor_volume: AtomicU32::new(monitor.volume.to_bits()),
        }
    }

    pub fn apply_monitor(&self, monitor: &Monitor) {
        self.monitor_enabled
            .store(monitor.enabled, Ordering::Relaxed);
        self.monitor_volume
            .store(monitor.volume.to_bits(), Ordering::Relaxed);
    }

    pub fn toggle_monitor(&self) -> bool {
        let prev = self.monitor_enabled.load(Ordering::Relaxed);
        self.monitor_enabled.store(!prev, Ordering::Relaxed);
        !prev
    }

    pub fn monitor_enabled(&self) -> bool {
        self.monitor_enabled.load(Ordering::Relaxed)
    }

    pub fn set_monitor_enabled(&self, enabled: bool) {
        self.monitor_enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn monitor_volume(&self) -> f32 {
        f32::from_bits(self.monitor_volume.load(Ordering::Relaxed))
    }
}

/// Unity path to VB-Cable (no user gain stage).
pub fn copy_unity_voice(voice: &[f32], out: &mut [f32]) {
    debug_assert_eq!(voice.len(), out.len());
    for i in 0..out.len() {
        out[i] = soft_clip(voice[i]);
    }
}

/// Scale monitor bus samples by headphone listen volume.
pub fn apply_monitor_volume(controls: &MixControls, samples: &mut [f32]) {
    let volume = controls.monitor_volume();
    if (volume - 1.0).abs() < f32::EPSILON {
        for s in samples.iter_mut() {
            *s = soft_clip(*s);
        }
        return;
    }
    for s in samples.iter_mut() {
        *s = soft_clip(*s * volume);
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
    fn monitor_volume_scales_samples() {
        let controls = MixControls::from_monitor(&Monitor {
            enabled: true,
            volume: 0.5,
        });
        let mut samples = [1.0f32, -1.0];
        apply_monitor_volume(&controls, &mut samples);
        assert_eq!(samples, [0.5, -0.5]);
    }
}
