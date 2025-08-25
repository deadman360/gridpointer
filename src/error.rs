//! Error types for GridPointer

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GridPointerError {
    #[error("Wayland connection error: {0}")]
    Wayland(String),

    #[error("Input device error: {0}")]
    Input(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Motion controller error: {0}")]
    Motion(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] toml::de::Error),
}

pub type Result<T> = std::result::Result<T, GridPointerError>;
