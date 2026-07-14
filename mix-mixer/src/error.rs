use std::fmt;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("config: {0}")]
    Config(String),

    #[error("device: {0}")]
    Device(String),

    #[error("audio: {0}")]
    Audio(String),

    #[error("soundboard: {0}")]
    Soundboard(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("hotkey: {0}")]
    Hotkey(String),

    #[error("tray: {0}")]
    Tray(String),
}

impl Error {
    pub fn config(msg: impl fmt::Display) -> Self {
        Self::Config(msg.to_string())
    }

    pub fn device(msg: impl fmt::Display) -> Self {
        Self::Device(msg.to_string())
    }

    pub fn audio(msg: impl fmt::Display) -> Self {
        Self::Audio(msg.to_string())
    }

    pub fn soundboard(msg: impl fmt::Display) -> Self {
        Self::Soundboard(msg.to_string())
    }
}
