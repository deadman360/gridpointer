//! Movement controller with smooth easing and dash support

use crate::input::Direction;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tracing::debug;

use crate::config::Config;

/// Motion events for the controller
#[derive(Debug, Clone)]
pub enum MotionEvent {
    Move { direction: Direction },
    Dash { direction: Direction },
}

/// Current motion state
#[derive(Debug, Clone, PartialEq)]
enum MotionState {
    Idle,
    Moving {
        from: (f64, f64),
        to: (f64, f64),
        start_time: Instant,
        duration: Duration,
    },
}

/// Motion controller with easing support
pub struct MotionController {
    config: Arc<RwLock<Config>>,
    state: MotionState,
    current_grid_pos: (u32, u32),
    current_screen_pos: (f64, f64),
}

impl MotionController {
    pub fn new(config: Arc<RwLock<Config>>) -> Self {
        Self {
            config,
            state: MotionState::Idle,
            current_grid_pos: (0, 0),
            current_screen_pos: (0.5, 0.5), // Start at screen center
        }
    }

    /// Handle motion events
    pub fn handle_event(&mut self, event: MotionEvent) {
        let config = match self.config.try_read() {
            Ok(config) => config,
            Err(_) => return,
        };

        let (new_grid_pos, duration) = match event {
            MotionEvent::Move { direction } => {
                let new_pos = self.apply_direction(self.current_grid_pos, direction, 1, &config);
                (new_pos, Duration::from_millis(config.movement.tween_ms))
            }
            MotionEvent::Dash { direction } => {
                let new_pos = self.apply_direction(
                    self.current_grid_pos,
                    direction,
                    config.movement.dash_cells,
                    &config,
                );
                (new_pos, Duration::from_millis(config.movement.tween_ms))
            }
        };

        if new_grid_pos != self.current_grid_pos {
            let from = self.current_screen_pos;
            let to = self.grid_to_screen(new_grid_pos, &config);

            self.state = MotionState::Moving {
                from,
                to,
                start_time: Instant::now(),
                duration,
            };
            self.current_grid_pos = new_grid_pos;

            debug!("Moving from {:?} to {:?}", from, to);
        }
    }

    /// Update motion state and return current screen position if changed
    pub fn update(&mut self) -> Option<(f64, f64)> {
        match &self.state {
            MotionState::Idle => None,
            MotionState::Moving {
                from,
                to,
                start_time,
                duration,
            } => {
                let elapsed = start_time.elapsed();

                if elapsed >= *duration {
                    self.current_screen_pos = *to;
                    self.state = MotionState::Idle;
                    Some(*to)
                } else {
                    let progress = elapsed.as_secs_f64() / duration.as_secs_f64();
                    let eased_progress = ease_out_cubic(progress);

                    let x = from.0 + (to.0 - from.0) * eased_progress;
                    let y = from.1 + (to.1 - from.1) * eased_progress;

                    self.current_screen_pos = (x, y);
                    Some((x, y))
                }
            }
        }
    }

    fn apply_direction(
        &self,
        pos: (u32, u32),
        direction: Direction,
        distance: u32,
        config: &Config,
    ) -> (u32, u32) {
        let (x, y) = pos;
        match direction {
            Direction::Up => (x, y.saturating_sub(distance)),
            Direction::Down => (x, (y + distance).min(config.grid.rows - 1)),
            Direction::Left => (x.saturating_sub(distance), y),
            Direction::Right => ((x + distance).min(config.grid.cols - 1), y),
        }
    }

    fn grid_to_screen(&self, grid_pos: (u32, u32), config: &Config) -> (f64, f64) {
        let x = grid_pos.0 as f64 / (config.grid.cols - 1) as f64;
        let y = grid_pos.1 as f64 / (config.grid.rows - 1) as f64;
        (x, y)
    }
}

/// Cubic ease-out function for smooth movement
fn ease_out_cubic(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t).powi(3)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ease_out_cubic() {
        assert_eq!(ease_out_cubic(0.0), 0.0);
        assert_eq!(ease_out_cubic(1.0), 1.0);
        assert!(ease_out_cubic(0.5) > 0.5); // Should be faster than linear
        assert!(ease_out_cubic(0.2) < 0.4); // Should be slower at start
    }

    #[tokio::test]
    async fn test_motion_controller() {
        use crate::config::{Config, DisplayConfig, GridConfig, InputConfig, MovementConfig};

        let config = Config {
            grid: GridConfig { cols: 10, rows: 10 },
            movement: MovementConfig {
                dash_cells: 3,
                tween_ms: 100,
            },
            input: InputConfig {
                keyboard_device: None,
                gamepad_device: None,
            },
            display: DisplayConfig {
                target_monitor: "auto".to_string(),
            },
        };

        let mut controller = MotionController::new(Arc::new(RwLock::new(config)));

        // Test normal movement
        controller.handle_event(MotionEvent::Move {
            direction: Direction::Right,
        });
        assert_eq!(controller.current_grid_pos, (1, 0));

        // Test dash movement
        controller.handle_event(MotionEvent::Dash {
            direction: Direction::Right,
        });
        assert_eq!(controller.current_grid_pos, (4, 0));
    }
}
