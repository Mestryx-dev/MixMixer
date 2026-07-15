//! UI string catalog and locale resolution.
//!
//! Language is resolved in this order:
//! 1. `MIXMIXER_LANG` environment variable (`en` or `fr`)
//! 2. `locale` field in `config.json`
//! 3. English (`en`) as the default

mod en;
mod fr;

use serde::{Deserialize, Serialize};

/// Supported UI languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Locale {
    #[default]
    En,
    Fr,
}

impl Locale {
    /// Parse a locale code (`en`, `fr`, case-insensitive).
    pub fn parse(code: &str) -> Option<Self> {
        match code.trim().to_ascii_lowercase().as_str() {
            "en" | "english" => Some(Self::En),
            "fr" | "french" | "français" | "francais" => Some(Self::Fr),
            _ => None,
        }
    }

    /// Resolve the active locale from environment and config.
    pub fn resolve(config_locale: Option<&str>) -> Self {
        if let Ok(env) = std::env::var("MIXMIXER_LANG") {
            if let Some(locale) = Self::parse(&env) {
                return locale;
            }
        }
        if let Some(code) = config_locale {
            if let Some(locale) = Self::parse(code) {
                return locale;
            }
        }
        Self::default()
    }

    /// Return the string table for this locale.
    pub fn texts(&self) -> &'static UiText {
        match self {
            Self::En => &en::TEXTS,
            Self::Fr => &fr::TEXTS,
        }
    }

    /// Serialize as a config-friendly code.
    pub fn code(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Fr => "fr",
        }
    }
}

/// All user-visible UI strings for a locale.
pub struct UiText {
    pub header_subtitle: &'static str,
    pub status_active: &'static str,
    pub status_reconnecting: &'static str,
    pub status_inactive: &'static str,
    pub btn_quit: &'static str,
    pub btn_cancel: &'static str,
    pub btn_apply: &'static str,
    pub section_routing: &'static str,
    pub section_devices: &'static str,
    pub section_audio: &'static str,
    pub routing_enable: &'static str,
    pub device_mic: &'static str,
    pub device_vac: &'static str,
    pub device_monitor: &'static str,
    pub monitor_headphones: &'static str,
    pub gain_voice: &'static str,
    pub gain_master: &'static str,
    pub buffer_latency: &'static str,
    pub buffer_hint: &'static str,
    pub status_apply_ok: &'static str,
    pub status_routing_on: &'static str,
    pub status_routing_off: &'static str,
    pub status_send_failed: &'static str,
    pub status_command_failed: &'static str,
    pub window_title: &'static str,
    pub tray_settings: &'static str,
    pub tray_toggle_monitor: &'static str,
    pub tray_reload: &'static str,
    pub tray_quit: &'static str,
    pub tray_tooltip: &'static str,
}
