//! GridPointer library interface
//!
//! This module provides a clean API for embedding GridPointer functionality
//! into other applications or for creating custom implementations.

pub mod config;
pub mod error;
pub mod input;
pub mod motion;
pub mod wl;

pub use config::{Config, ConfigManager};
pub use error::{GridPointerError, Result};
pub use input::{Direction, InputEvent, InputManager};
pub use motion::{MotionController, MotionEvent};
pub use wl::WaylandManager;

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT_HASH: &str = match option_env!("GIT_HASH") {
    Some(hash) => hash,
    None => "unknown",
};
pub const BUILD_TIMESTAMP: &str = match option_env!("BUILD_TIMESTAMP") {
    Some(ts) => ts,
    None => "unknown",
};

/// Get version string with git hash and build info
pub fn version_string() -> String {
    format!(
        "GridPointer {} ({}), built {}",
        VERSION, GIT_HASH, BUILD_TIMESTAMP
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        assert!(!VERSION.is_empty());
        assert!(!version_string().is_empty());
        println!("Version: {}", version_string());
    }
}
