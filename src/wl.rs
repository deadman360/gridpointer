//! Wayland integration for virtual pointer control

use crate::error::{GridPointerError, Result};
use std::sync::{Arc, Mutex};
use tracing::{debug, info, warn};
use wayland_client::{
    protocol::{wl_compositor, wl_output, wl_registry, wl_seat},
    Connection, Dispatch, QueueHandle, EventQueue,
};
use wayland_protocols_wlr::virtual_pointer::v1::client::{
    zwlr_virtual_pointer_manager_v1, zwlr_virtual_pointer_v1,
};

/// Wayland manager for virtual pointer control
pub struct WaylandManager {
    connection: Connection,
    virtual_pointer: Option<zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1>,
    outputs: Vec<OutputInfo>,
    queue: Arc<Mutex<EventQueue<AppState>>>,
}

#[derive(Debug, Clone)]
struct OutputInfo {
    output: wl_output::WlOutput,
    name: String,
    width: i32,
    height: i32,
    scale: i32,
}

struct AppState {
    outputs: Vec<OutputInfo>,
    virtual_pointer_manager: Option<zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1>,
    seat: Option<wl_seat::WlSeat>,
}

impl WaylandManager {
    pub async fn new() -> anyhow::Result<Self> {
        info!("Initializing Wayland connection");
        
        let connection = Connection::connect_to_env()?;
        let display = connection.display();
        
        let mut event_queue = connection.new_event_queue();
        let qh = event_queue.handle();
        
        let state = AppState {
            outputs: Vec::new(),
            virtual_pointer_manager: None,
            seat: None,
        };
        
        let registry = display.get_registry(&qh, ());
        
        // Initial roundtrip to get globals
        event_queue.blocking_dispatch(&mut state)?;
        
        let virtual_pointer = if let (Some(manager), Some(seat)) = 
            (&state.virtual_pointer_manager, &state.seat) {
            Some(manager.create_virtual_pointer(Some(seat), &qh, ()))
        } else {
            warn!("Virtual pointer manager or seat not available");
            None
        };
        
        Ok(Self {
            connection,
            virtual_pointer,
            outputs: state.outputs,
            queue: Arc::new(Mutex::new(event_queue)),
        })
    }
    
    /// Move cursor to normalized screen coordinates (0.0-1.0)
    pub async fn move_cursor(&self, x: f64, y: f64) -> Result<()> {
        if let Some(pointer) = &self.virtual_pointer {
            // Get primary output dimensions
            let (width, height) = self.get_primary_output_size();
            
            // Convert normalized coordinates to absolute pixels
            let abs_x = (x * width as f64) as u32;
            let abs_y = (y * height as f64) as u32;
            
            pointer.motion_absolute(
                0, // time
                abs_x,
                abs_y,
                width as u32,
                height as u32,
            );
            pointer.frame();
            
            // Flush the connection
            if let Ok(mut queue) = self.queue.lock() {
                let _ = queue.flush();
            }
            
            debug!("Moved cursor to ({:.3}, {:.3}) -> ({}, {})", x, y, abs_x, abs_y);
        }
        Ok(())
    }
    
    /// Perform left mouse click
    pub async fn click_left(&self) -> Result<()> {
        if let Some(pointer) = &self.virtual_pointer {
            // Press
            pointer.button(0, 0x110, 1); // BTN_LEFT = 0x110
            pointer.frame();
            
            // Release
            pointer.button(1, 0x110, 0);
            pointer.frame();
            
            // Flush
            if let Ok(mut queue) = self.queue.lock() {
                let _ = queue.flush();
            }
            
            debug!("Left click performed");
        }
        Ok(())
    }
    
    fn get_primary_output_size(&self) -> (i32, i32) {
        if let Some(output) = self.outputs.first() {
            (output.width, output.height)
        } else {
            // Fallback dimensions
            (1920, 1080)
        }
    }
}

// Wayland protocol implementations
impl Dispatch<wl_registry::WlRegistry, ()> for AppState {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<AppState>,
    ) {
        if let wl_registry::Event::Global { name, interface, version } = event {
            match &interface[..] {
                "zwlr_virtual_pointer_manager_v1" => {
                    let manager = registry.bind::<zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1, _, _>(
                        name, version.min(1), qh, ()
                    );
                    state.virtual_pointer_manager = Some(manager);
                }
                "wl_seat" => {
                    let seat = registry.bind::<wl_seat::WlSeat, _, _>(name, version.min(1), qh, ());
                    state.seat = Some(seat);
                }
                "wl_output" => {
                    let output = registry.bind::<wl_output::WlOutput, _, _>(name, version.min(2), qh, ());
                    state.outputs.push(OutputInfo {
                        output,
                        name: format!("output-{}", name),
                        width: 1920,
                        height: 1080,
                        scale: 1,
                    });
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1,
        _: zwlr_virtual_pointer_manager_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppState>,
    ) {
        // No events for manager
    }
}

impl Dispatch<zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1,
        _: zwlr_virtual_pointer_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppState>,
    ) {
        // No events for virtual pointer
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &wl_seat::WlSeat,
        _: wl_seat::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppState>,
    ) {
        // Handle seat events if needed
    }
}

impl Dispatch<wl_output::WlOutput, ()> for AppState {
    fn event(
        state: &mut Self,
        output: &wl_output::WlOutput,
        event: wl_output::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppState>,
    ) {
        if let Some(output_info) = state.outputs.iter_mut().find(|o| &o.output == output) {
            match event {
                wl_output::Event::Mode { width, height, .. } => {
                    output_info.width = width;
                    output_info.height = height;
                }
                wl_output::Event::Scale { factor } => {
                    output_info.scale = factor;
                }
                wl_output::Event::Name { name } => {
                    output_info.name = name;
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<wl_compositor::WlCompositor, ()> for AppState {
    fn event(
        _: &mut Self,
        _: &wl_compositor::WlCompositor,
        _: wl_compositor::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<AppState>,
    ) {
        // No events needed
    }
}
`
