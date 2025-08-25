//! Input handling for keyboard and gamepad devices

use crate::config::Config;
use crate::error::{GridPointerError, Result};
use evdev::{Device, EventType, InputEventTrait, Key};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast, mpsc};
use tokio::time::{Duration, interval};
use tracing::{debug, info, warn};

/// Direction for movement
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Input events from keyboard or gamepad
#[derive(Debug, Clone)]
pub enum InputEvent {
    Move { direction: Direction, dash: bool },
    Click,
    Quit,
}

/// Input device manager
pub struct InputManager {
    keyboard_device: Option<Device>,
    gamepad_device: Option<Device>,
    key_states: HashMap<Key, bool>,
}

impl InputManager {
    pub async fn new(config: &Arc<RwLock<Config>>) -> anyhow::Result<Self> {
        let config = config.read().await;

        let keyboard_device = match &config.input.keyboard_device {
            Some(path) => Some(Self::open_device(path)?),
            None => Self::find_keyboard_device()?,
        };

        let gamepad_device = match &config.input.gamepad_device {
            Some(path) => Some(Self::open_device(path)?),
            None => Self::find_gamepad_device()?,
        };

        if keyboard_device.is_none() && gamepad_device.is_none() {
            return Err(
                GridPointerError::Input("No suitable input devices found".to_string()).into(),
            );
        }

        info!("Input devices initialized");
        if keyboard_device.is_some() {
            info!("  Keyboard: enabled");
        }
        if gamepad_device.is_some() {
            info!("  Gamepad: enabled");
        }

        Ok(Self {
            keyboard_device,
            gamepad_device,
            key_states: HashMap::new(),
        })
    }

    /// Main input loop
    pub async fn run(
        mut self,
        tx: mpsc::UnboundedSender<InputEvent>,
        mut shutdown: broadcast::Receiver<()>,
    ) -> anyhow::Result<()> {
        let mut poll_timer = interval(Duration::from_millis(1));

        loop {
            tokio::select! {
                _ = poll_timer.tick() => {
                    if let Err(e) = self.poll_events(&tx).await {
                        warn!("Input polling error: {}", e);
                    }
                }
                _ = shutdown.recv() => {
                    break;
                }
            }
        }

        Ok(())
    }

    async fn poll_events(&mut self, tx: &mpsc::UnboundedSender<InputEvent>) -> Result<()> {
        // Poll keyboard
        if let Some(device) = &mut self.keyboard_device {
            while let Ok(events) = device.fetch_events() {
                for event in events {
                    if let Some(input_event) = self.handle_keyboard_event(event)? {
                        let _ = tx.send(input_event);
                    }
                }
            }
        }

        // Poll gamepad
        if let Some(device) = &mut self.gamepad_device {
            while let Ok(events) = device.fetch_events() {
                for event in events {
                    if let Some(input_event) = self.handle_gamepad_event(event)? {
                        let _ = tx.send(input_event);
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_keyboard_event(&mut self, event: evdev::InputEvent) -> Result<Option<InputEvent>> {
        if event.event_type() != EventType::KEY {
            return Ok(None);
        }

        let key = Key::new(event.code());
        let pressed = event.value() == 1;
        let released = event.value() == 0;

        // Update key states
        if pressed || released {
            self.key_states.insert(key, pressed);
        }

        if pressed {
            match key {
                Key::KEY_UP => {
                    let dash = self.key_states.get(&Key::KEY_LEFTSHIFT).unwrap_or(&false)
                        || self.key_states.get(&Key::KEY_RIGHTSHIFT).unwrap_or(&false);
                    return Ok(Some(InputEvent::Move {
                        direction: Direction::Up,
                        dash,
                    }));
                }
                Key::KEY_DOWN => {
                    let dash = self.key_states.get(&Key::KEY_LEFTSHIFT).unwrap_or(&false)
                        || self.key_states.get(&Key::KEY_RIGHTSHIFT).unwrap_or(&false);
                    return Ok(Some(InputEvent::Move {
                        direction: Direction::Down,
                        dash,
                    }));
                }
                Key::KEY_LEFT => {
                    let dash = self.key_states.get(&Key::KEY_LEFTSHIFT).unwrap_or(&false)
                        || self.key_states.get(&Key::KEY_RIGHTSHIFT).unwrap_or(&false);
                    return Ok(Some(InputEvent::Move {
                        direction: Direction::Left,
                        dash,
                    }));
                }
                Key::KEY_RIGHT => {
                    let dash = self.key_states.get(&Key::KEY_LEFTSHIFT).unwrap_or(&false)
                        || self.key_states.get(&Key::KEY_RIGHTSHIFT).unwrap_or(&false);
                    return Ok(Some(InputEvent::Move {
                        direction: Direction::Right,
                        dash,
                    }));
                }
                Key::KEY_SPACE => {
                    return Ok(Some(InputEvent::Click));
                }
                Key::KEY_ESC => {
                    return Ok(Some(InputEvent::Quit));
                }
                _ => {}
            }
        }

        Ok(None)
    }

    fn handle_gamepad_event(&mut self, event: evdev::InputEvent) -> Result<Option<InputEvent>> {
        // Simplified gamepad handling - implement based on your gamepad type
        if event.event_type() == EventType::KEY {
            let key = Key::new(event.code());
            let pressed = event.value() == 1;

            if pressed {
                match key {
                    Key::BTN_A => return Ok(Some(InputEvent::Click)),
                    Key::BTN_START => return Ok(Some(InputEvent::Quit)),
                    _ => {}
                }
            }
        }

        Ok(None)
    }

    fn open_device<P: AsRef<Path>>(path: P) -> Result<Device> {
        Device::open(path)
            .map_err(|e| GridPointerError::Input(format!("Failed to open device: {}", e)))
    }

    fn find_keyboard_device() -> Result<Option<Device>> {
        for path in evdev::enumerate() {
            if let Ok(device) = Device::open(&path.1) {
                if device.supported_keys().map_or(false, |keys| {
                    keys.contains(Key::KEY_A) && keys.contains(Key::KEY_SPACE)
                }) {
                    debug!("Found keyboard device: {}", path.1.display());
                    return Ok(Some(device));
                }
            }
        }
        Ok(None)
    }

    fn find_gamepad_device() -> Result<Option<Device>> {
        for path in evdev::enumerate() {
            if let Ok(device) = Device::open(&path.1) {
                if device.supported_keys().map_or(false, |keys| {
                    keys.contains(Key::BTN_A) && keys.contains(Key::BTN_B)
                }) {
                    debug!("Found gamepad device: {}", path.1.display());
                    return Ok(Some(device));
                }
            }
        }
        Ok(None)
    }
}
