use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::devices;
use crate::error::{Error, Result};
use crate::i18n::Locale;

const APP_DIR_NAME: &str = "MixMixer";
const CONFIG_FILE_NAME: &str = "config.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// UI language (`en` or `fr`). Overridden by `MIXMIXER_LANG` at runtime.
    #[serde(default)]
    pub locale: Locale,

    #[serde(default = "default_sample_rate")]
    pub sample_rate: u32,

    #[serde(default = "default_buffer_frames")]
    pub buffer_frames: u32,

    pub devices: DeviceNames,

    #[serde(default)]
    pub gains: Gains,

    #[serde(default)]
    pub monitor: Monitor,

    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceNames {
    pub voice_input: String,
    pub virtual_mic_output: String,
    pub monitor_output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Gains {
    #[serde(default = "default_one")]
    pub voice: f32,
    #[serde(default = "default_one")]
    pub master: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Monitor {
    #[serde(default)]
    pub enabled: bool,
}

impl Config {
    /// `%APPDATA%\MixMixer\config.json` (Windows user settings).
    pub fn default_path() -> Result<PathBuf> {
        let appdata = std::env::var_os("APPDATA").ok_or_else(|| {
            Error::config("APPDATA is not set — cannot resolve user config directory")
        })?;
        Ok(PathBuf::from(appdata)
            .join(APP_DIR_NAME)
            .join(CONFIG_FILE_NAME))
    }

    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| Error::config(format!("read {}: {e}", path.display())))?;
        let cfg: Config = serde_json::from_str(&text)?;
        cfg.validate()?;
        Ok(cfg)
    }

    /// Load config, or create a smart default under AppData on first run.
    pub fn load_or_create(path: &Path) -> Result<(Self, bool)> {
        if path.exists() {
            return Ok((Self::load(path)?, false));
        }
        let cfg = Self::generate_default()?;
        cfg.save(path)?;
        info!(path = %path.display(), "created default config");
        Ok((cfg, true))
    }

    /// Defaults from the live device list (system mic + CABLE Input when present).
    pub fn generate_default() -> Result<Self> {
        let (voice_input, virtual_mic_output, monitor_output) =
            devices::suggest_default_devices()?;
        Ok(Self {
            locale: Locale::from_system(),
            sample_rate: default_sample_rate(),
            buffer_frames: default_buffer_frames(),
            devices: DeviceNames {
                voice_input,
                virtual_mic_output,
                monitor_output,
            },
            gains: Gains::default(),
            monitor: Monitor::default(),
            enabled: default_enabled(),
        })
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        self.validate()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                Error::config(format!("create config directory {}: {e}", parent.display()))
            })?;
        }
        let text = serde_json::to_string_pretty(self)?;
        std::fs::write(path, text)
            .map_err(|e| Error::config(format!("write {}: {e}", path.display())))?;
        Ok(())
    }

    fn validate(&self) -> Result<()> {
        if self.sample_rate == 0 {
            return Err(Error::config("sample_rate must be > 0"));
        }
        if self.buffer_frames == 0 {
            return Err(Error::config("buffer_frames must be > 0"));
        }
        if self.devices.voice_input.is_empty() {
            return Err(Error::config("devices.voice_input is required"));
        }
        if self.devices.virtual_mic_output.is_empty() {
            return Err(Error::config("devices.virtual_mic_output is required"));
        }
        Ok(())
    }

    pub fn needs_stream_restart(&self, other: &Self) -> bool {
        self.devices != other.devices
            || self.sample_rate != other.sample_rate
            || self.buffer_frames != other.buffer_frames
            || self.monitor.enabled != other.monitor.enabled
    }
}

impl Default for Gains {
    fn default() -> Self {
        Self {
            voice: 1.0,
            master: 1.0,
        }
    }
}

impl Default for Monitor {
    fn default() -> Self {
        Self { enabled: false }
    }
}

fn default_sample_rate() -> u32 {
    48_000
}

fn default_buffer_frames() -> u32 {
    128
}

fn default_one() -> f32 {
    1.0
}

fn default_enabled() -> bool {
    true
}
