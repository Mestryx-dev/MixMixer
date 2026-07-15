//! UI string catalog and locale resolution.
//!
//! The in-app **Language** dropdown (General section) is the primary way to switch UI
//! language. The choice is saved to `config.json`. Optional override: `MIXMIXER_LANG`.

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

    /// Two-letter UI code (minimal locale picker).
    pub fn short_code(self) -> &'static str {
        match self {
            Self::En => "EN",
            Self::Fr => "FR",
        }
    }

    /// Human-readable name shown in the language picker.
    pub fn display_name(self) -> &'static str {
        match self {
            Self::En => "English",
            Self::Fr => "Français",
        }
    }

    /// All supported locales (for dropdown menus).
    pub const ALL: [Locale; 2] = [Self::En, Self::Fr];
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
    pub footer_unsaved: &'static str,
    pub section_general: &'static str,
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
    pub tray_about: &'static str,
    pub tray_quit: &'static str,
    pub tray_tooltip: &'static str,
    pub about_title: &'static str,
    pub about_description: &'static str,
    pub about_url: &'static str,
}
