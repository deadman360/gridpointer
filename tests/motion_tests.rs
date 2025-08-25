//! Tests for motion controller

use gridpointer::{
    config::{Config, DisplayConfig, GridConfig, InputConfig, MovementConfig},
    input::Direction,
    motion::{MotionController, MotionEvent},
};
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_basic_movement() {
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

    // Start at center
    assert_eq!(controller.current_grid_pos, (0, 0));

    // Move right
    controller.handle_event(MotionEvent::Move {
        direction: Direction::Right,
    });
    assert_eq!(controller.current_grid_pos, (1, 0));

    // Move down
    controller.handle_event(MotionEvent::Move {
        direction: Direction::Down,
    });
    assert_eq!(controller.current_grid_pos, (1, 1));

    // Move left
    controller.handle_event(MotionEvent::Move {
        direction: Direction::Left,
    });
    assert_eq!(controller.current_grid_pos, (0, 1));

    // Move up
    controller.handle_event(MotionEvent::Move {
        direction: Direction::Up,
    });
    assert_eq!(controller.current_grid_pos, (0, 0));
}

#[tokio::test]
async fn test_dash_movement() {
    let config = Config {
        grid: GridConfig { cols: 20, rows: 20 },
        movement: MovementConfig {
            dash_cells: 5,
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

    // Dash right 5 cells
    controller.handle_event(MotionEvent::Dash {
        direction: Direction::Right,
    });
    assert_eq!(controller.current_grid_pos, (5, 0));

    // Dash down 5 cells
    controller.handle_event(MotionEvent::Dash {
        direction: Direction::Down,
    });
    assert_eq!(controller.current_grid_pos, (5, 5));
}

#[tokio::test]
async fn test_boundary_conditions() {
    let config = Config {
        grid: GridConfig { cols: 5, rows: 5 },
        movement: MovementConfig {
            dash_cells: 10,
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

    // Try to dash beyond boundary
    controller.handle_event(MotionEvent::Dash {
        direction: Direction::Right,
    });
    assert_eq!(controller.current_grid_pos, (4, 0)); // Clamped to max

    // Try to move left beyond boundary
    controller.current_grid_pos = (0, 0);
    controller.handle_event(MotionEvent::Move {
        direction: Direction::Left,
    });
    assert_eq!(controller.current_grid_pos, (0, 0)); // Should stay at 0
}

#[test]
fn test_easing_function() {
    use gridpointer::motion::ease_out_cubic;

    // Test boundary conditions
    assert_eq!(ease_out_cubic(0.0), 0.0);
    assert_eq!(ease_out_cubic(1.0), 1.0);

    // Test that it's faster than linear at the end
    assert!(ease_out_cubic(0.5) > 0.5);

    // Test monotonicity
    assert!(ease_out_cubic(0.3) < ease_out_cubic(0.7));
}
