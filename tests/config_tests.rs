//! Tests for configuration management

use gridpointer::config::{Config, ConfigManager, GridConfig, MovementConfig};
use std::fs;
use tempfile::TempDir;

#[tokio::test]
async fn test_default_config() {
    let config = Config::default();
    assert_eq!(config.grid.cols, 20);
    assert_eq!(config.grid.rows, 12);
    assert_eq!(config.movement.dash_cells, 5);
    assert_eq!(config.movement.tween_ms, 150);
}

#[tokio::test]
async fn test_config_serialization() {
    let config = Config::default();
    let toml_str = toml::to_string(&config).unwrap();
    let parsed: Config = toml::from_str(&toml_str).unwrap();

    assert_eq!(config.grid.cols, parsed.grid.cols);
    assert_eq!(config.grid.rows, parsed.grid.rows);
    assert_eq!(config.movement.dash_cells, parsed.movement.dash_cells);
}
