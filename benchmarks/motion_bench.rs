//! Performance benchmarks for motion controller

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use gridpointer::{
    config::{Config, DisplayConfig, GridConfig, InputConfig, MovementConfig},
    input::Direction,
    motion::{MotionController, MotionEvent},
};
use std::sync::Arc;
use tokio::sync::RwLock;

fn motion_controller_benchmark(c: &mut Criterion) {
    let config = Config {
        grid: GridConfig {
            cols: 100,
            rows: 100,
        },
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

    c.bench_function("motion_event_handling", |b| {
        b.iter(|| {
            controller.handle_event(black_box(MotionEvent::Move {
                direction: Direction::Right,
            }));
        })
    });

    c.bench_function("motion_update_360hz", |b| {
        // First trigger some movement
        controller.handle_event(MotionEvent::Move {
            direction: Direction::Right,
        });

        b.iter(|| {
            black_box(controller.update());
        })
    });

    c.bench_function("dash_movement", |b| {
        b.iter(|| {
            controller.handle_event(black_box(MotionEvent::Dash {
                direction: Direction::Right,
            }));
        })
    });
}

fn easing_benchmark(c: &mut Criterion) {
    c.bench_function("ease_out_cubic", |b| {
        b.iter(|| {
            for i in 0..=1000 {
                let t = i as f64 / 1000.0;
                black_box(gridpointer::motion::ease_out_cubic(t));
            }
        })
    });
}

criterion_group!(benches, motion_controller_benchmark, easing_benchmark);
criterion_main!(benches);
