//! Configuration management with hot-reload support

use crate::error::{GridPointerError, Result};
use anyhow::Context;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use tracing::{info, warn};

/// Main configuration structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub grid: GridConfig,
    pub movement: MovementConfig,
    pub input: InputConfig,
    pub display: DisplayConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GridConfig {
    pub cols: u32,
    pub rows: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MovementConfig {
    pub dash_cells: u32,
    pub tween_ms: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InputConfig {
    pub keyboard_device: Option<String>,
    pub gamepad_device: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DisplayConfig {
    pub target_monitor: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            grid: GridConfig { cols: 20, rows: 12 },
            movement: MovementConfig {
                dash_cells: 5,
                tween_ms: 150,
            },
            input: InputConfig {
                keyboard_device: None,
                gamepad_device: None,
            },
            display: DisplayConfig {
                target_monitor: "auto".to_string(),
            },
        }
    }
}

/// Configuration manager with hot-reload support
pub struct ConfigManager {
    config_path: PathBuf,
    config: Arc<RwLock<Config>>,
}

impl ConfigManager {
    pub async fn new() -> anyhow::Result<Self> {
        let config_path = Self::get_config_path()?;
        let config = Self::load_or_create_config(&config_path).await?;

        Ok(Self {
            config_path,
            config: Arc::new(RwLock::new(config)),
        })
    }

    pub fn get_config(&self) -> Arc<RwLock<Config>> {
        self.config.clone()
    }

    /// Watch for configuration file changes and reload automatically
    pub async fn watch_config(&self, mut shutdown: broadcast::Receiver<()>) -> anyhow::Result<()> {
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        let config_path = self.config_path.clone();
        let config = self.config.clone();

        // Setup file watcher
        let mut watcher: RecommendedWatcher = notify::Watcher::new(
            move |result: notify::Result<Event>| {
                if let Ok(event) = result {
                    if matches!(event.kind, EventKind::Modify(_)) {
                        let _ = tx.blocking_send(event);
                    }
                }
            },
            notify::Config::default(),
        )?;

        if let Some(parent) = config_path.parent() {
            watcher.watch(parent, RecursiveMode::NonRecursive)?;
        }

        loop {
            tokio::select! {
                Some(_event) = rx.recv() => {
                    match Self::load_config(&config_path).await {
                        Ok(new_config) => {
                            *config.write().await = new_config;
                            info!("Configuration reloaded");
                        }
                        Err(e) => {
                            warn!("Failed to reload config: {}", e);
                        }
                    }
                }
                _ = shutdown.recv() => {
                    break;
                }
            }
        }

        Ok(())
    }

    fn get_config_path() -> anyhow::Result<PathBuf> {
        let mut path = dirs::config_dir().context("Could not determine config directory")?;
        path.push("gridpointer");
        std::fs::create_dir_all(&path)?;
        path.push("config.toml");
        Ok(path)
    }

    async fn load_or_create_config(path: &PathBuf) -> anyhow::Result<Config> {
        if path.exists() {
            Self::load_config(path).await
        } else {
            let config = Config::default();
            Self::save_config(path, &config).await?;
            info!("Created default configuration at {}", path.display());
            Ok(config)
        }
    }

    async fn load_config(path: &PathBuf) -> anyhow::Result<Config> {
        let content = tokio::fs::read_to_string(path).await?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    async fn save_config(path: &PathBuf, config: &Config) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(config)?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }
}
