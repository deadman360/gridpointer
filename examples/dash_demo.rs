//! Dash movement demonstration

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
    println!("GridPointer Demo: Dash Movement");

    let config = Config {
        grid: GridConfig { cols: 20, rows: 12 },
        movement: MovementConfig {
            dash_cells: 7,
            tween_ms: 300,
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

    println!("Grid size: 20x12, Dash distance: 7 cells");
    println!("Starting at center");

    // Start at center
    controller.current_grid_pos = (10, 6);
    print_position(&controller);

    // Perform dash sequence
    let dashes = vec![
        Direction::Right,
        Direction::Down,
        Direction::Left,
        Direction::Up,
    ];

    for direction in dashes {
        println!("\nDashing {:?}...", direction);
        controller.handle_event(MotionEvent::Dash { direction });

        // Animate the dash
        let start_time = std::time::Instant::now();
        loop {
            if let Some(pos) = controller.update() {
                print!(
                    "  Position: ({:.3}, {:.3}) Grid: ({}, {})   \r",
                    pos.0, pos.1, controller.current_grid_pos.0, controller.current_grid_pos.1
                );
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
            } else {
                break; // Animation complete
            }

            if start_time.elapsed() > Duration::from_secs(2) {
                break; // Safety timeout
            }

            sleep(Duration::from_millis(8)).await; // ~120 FPS for smooth demo
        }
        println!();
        print_position(&controller);
    }

    println!("\nDash demo complete!");
    Ok(())
}

fn print_position(controller: &MotionController) {
    println!(
        "Current grid position: ({}, {})",
        controller.current_grid_pos.0, controller.current_grid_pos.1
    );
}
