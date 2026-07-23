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

    /// Best effort UI language for a newly created config (`MIXMIXER_LANG`, then system, then English).
    pub fn from_system() -> Self {
        if let Ok(env) = std::env::var("MIXMIXER_LANG") {
            if let Some(locale) = Self::parse(&env) {
                return locale;
            }
        }
        system_ui_locale().unwrap_or_default()
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

#[cfg(windows)]
fn system_ui_locale() -> Option<Locale> {
    use windows_sys::Win32::Globalization::GetUserDefaultLocaleName;

    let mut buf = [0u16; 85];
    let len = unsafe { GetUserDefaultLocaleName(buf.as_mut_ptr(), buf.len() as i32) };
    if len <= 1 {
        return None;
    }
    let name = String::from_utf16_lossy(&buf[..(len as usize - 1)]);
    if name.to_ascii_lowercase().starts_with("fr") {
        Some(Locale::Fr)
    } else if name.to_ascii_lowercase().starts_with("en") {
        Some(Locale::En)
    } else {
        None
    }
}

#[cfg(not(windows))]
fn system_ui_locale() -> Option<Locale> {
    for key in ["LC_ALL", "LANG"] {
        if let Ok(val) = std::env::var(key) {
            if let Some(locale) = Locale::parse(val.split(['.', '_']).next().unwrap_or(&val)) {
                return Some(locale);
            }
        }
    }
    None
}

/// All user-visible UI strings for a locale.
pub struct UiText {
    pub header_subtitle: &'static str,
    pub status_active: &'static str,
    pub status_reconnecting: &'static str,
    pub status_inactive: &'static str,
    pub btn_quit: &'static str,
    pub section_general: &'static str,
    pub section_routing: &'static str,
    pub section_devices: &'static str,
    pub section_audio: &'static str,
    pub routing_enable: &'static str,
    pub device_mic: &'static str,
    pub device_vac: &'static str,
    pub device_monitor: &'static str,
    pub monitor_headphones: &'static str,
    pub monitor_volume: &'static str,
    pub buffer_latency: &'static str,
    pub buffer_hint: &'static str,
    pub status_send_failed: &'static str,
    pub window_title: &'static str,
    pub tray_about: &'static str,
    pub tray_quit: &'static str,
    pub tray_tooltip: &'static str,
    pub about_title: &'static str,
    pub about_description: &'static str,
    pub about_url: &'static str,
}
