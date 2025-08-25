//! Grid movement demonstration

use gridpointer::{
    config::{Config, DisplayConfig, GridConfig, InputConfig, MovementConfig},
    input::Direction,
    motion::{MotionController, MotionEvent},
};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("GridPointer Demo: Basic Grid Movement");

    let config = Config {
        grid: GridConfig { cols: 10, rows: 6 },
        movement: MovementConfig {
            dash_cells: 3,
            tween_ms: 500,
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

    println!("Starting at grid position (0, 0)");
    print_grid_position(&controller);

    // Demonstrate basic movements
    let moves = vec![
        (Direction::Right, false, "Move right"),
        (Direction::Right, false, "Move right again"),
        (Direction::Down, false, "Move down"),
        (Direction::Down, false, "Move down again"),
        (Direction::Left, true, "Dash left"),
        (Direction::Up, true, "Dash up"),
    ];

    for (direction, is_dash, description) in moves {
        println!("\n{}", description);

        let event = if is_dash {
            MotionEvent::Dash { direction }
        } else {
            MotionEvent::Move { direction }
        };

        controller.handle_event(event);
        print_grid_position(&controller);

        // Simulate movement animation
        for _ in 0..10 {
            if let Some(pos) = controller.update() {
                print!("  Screen pos: ({:.3}, {:.3})\r", pos.0, pos.1);
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
            }
            sleep(Duration::from_millis(50)).await;
        }
        println!();
    }

    println!("\nDemo complete!");
    Ok(())
}

fn print_grid_position(controller: &MotionController) {
    println!(
        "Grid position: ({}, {})",
        controller.current_grid_pos.0, controller.current_grid_pos.1
    );
}
