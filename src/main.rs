//! GridPointer - Grid-based cursor control daemon for Wayland
//!
//! A high-performance daemon that provides smooth, game-like cursor movement
//! on a logical grid with configurable easing and dash support.

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tokio::time::{Duration, interval};
use tracing::{error, info, warn};

mod config;
mod error;
mod input;
mod motion;
mod wl;

use crate::config::{Config, ConfigManager};
use crate::input::{InputEvent, InputManager};
use crate::motion::{MotionController, MotionEvent};
use crate::wl::WaylandManager;

/// Main application state
pub struct GridPointer {
    config_manager: ConfigManager,
    input_manager: InputManager,
    motion_controller: MotionController,
    wayland_manager: WaylandManager,
}

impl GridPointer {
    /// Initialize the GridPointer daemon
    pub async fn new() -> Result<Self> {
        info!("Initializing GridPointer daemon");

        let config_manager = ConfigManager::new().await?;
        let config = config_manager.get_config();

        let wayland_manager = WaylandManager::new().await?;
        let input_manager = InputManager::new(&config).await?;
        let motion_controller = MotionController::new(config.clone());

        Ok(Self {
            config_manager,
            input_manager,
            motion_controller,
            wayland_manager,
        })
    }

    /// Main event loop running at 360 Hz
    pub async fn run(mut self) -> Result<()> {
        info!("Starting GridPointer main loop at 360 Hz");

        let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
        let (input_tx, mut input_rx) = mpsc::unbounded_channel();
        let (motion_tx, mut motion_rx) = mpsc::unbounded_channel();

        // Start input handling
        let input_shutdown = shutdown_tx.subscribe();
        let input_handle =
            tokio::spawn(async move { self.input_manager.run(input_tx, input_shutdown).await });

        // Start config hot-reload
        let config_shutdown = shutdown_tx.subscribe();
        let config_handle =
            tokio::spawn(async move { self.config_manager.watch_config(config_shutdown).await });

        // Main update loop at 360 Hz (≈2.78ms per frame)
        let mut update_timer = interval(Duration::from_micros(2778));
        update_timer.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

        loop {
            tokio::select! {
                // Handle input events
                Some(input_event) = input_rx.recv() => {
                    if let Err(e) = self.handle_input_event(input_event, &motion_tx, &shutdown_tx).await {
                        error!("Input event error: {}", e);
                    }
                }

                // Handle motion updates
                Some(motion_event) = motion_rx.recv() => {
                    if let Err(e) = self.handle_motion_event(motion_event).await {
                        error!("Motion event error: {}", e);
                    }
                }

                // 360 Hz update tick
                _ = update_timer.tick() => {
                    if let Some(position) = self.motion_controller.update() {
                        if let Err(e) = self.wayland_manager.move_cursor(position.0, position.1).await {
                            warn!("Cursor move error: {}", e);
                        }
                    }
                }

                // Shutdown signal
                _ = shutdown_rx.recv() => {
                    info!("Shutdown signal received");
                    break;
                }
            }
        }

        // Cleanup
        let _ = input_handle.await;
        let _ = config_handle.await;

        info!("GridPointer daemon stopped");
        Ok(())
    }

    async fn handle_input_event(
        &mut self,
        event: InputEvent,
        motion_tx: &mpsc::UnboundedSender<MotionEvent>,
        shutdown_tx: &broadcast::Sender<()>,
    ) -> Result<()> {
        match event {
            InputEvent::Move { direction, dash } => {
                let motion_event = if dash {
                    MotionEvent::Dash { direction }
                } else {
                    MotionEvent::Move { direction }
                };
                let _ = motion_tx.send(motion_event);
            }
            InputEvent::Click => {
                self.wayland_manager.click_left().await?;
            }
            InputEvent::Quit => {
                let _ = shutdown_tx.send(());
            }
        }
        Ok(())
    }

    async fn handle_motion_event(&mut self, event: MotionEvent) -> Result<()> {
        self.motion_controller.handle_event(event);
        Ok(())
    }
}
