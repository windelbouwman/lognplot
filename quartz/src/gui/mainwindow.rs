//! This file will construct the main application onto a window.
//! Use this to actually run the mainapp instance.
//! This file will:
//! - Create a vulkan instance
//! - Create a vulkan surface
//! - Display a window
//! - Run the event / draw loop

use std::time::Instant;
use vulkano_win::VkSurfaceBuild;
use winit::{Event, EventsLoop, WindowBuilder, WindowEvent};

use super::backends::vulkan::{self as vulkan_backend, enable_logging, VulkanEngine};
use super::mainapp::MainApp;

pub fn run_gui(mut app: MainApp) {
    info!("Starting gui!!");

    let instance = vulkan_backend::create_vulkan_instance();
    enable_logging(&instance);

    // Create window with event loop:
    let mut events_loop = EventsLoop::new();
    let surface = WindowBuilder::new()
        .with_title("Quartz petabyte tracer")
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();
    let _window = surface.window();

    let mut vulkan_engine = VulkanEngine::new(instance, surface);

    let mut t1 = Instant::now();

    loop {
        // Proceed one tick:
        app.tick();

        // Render app in vulkan:
        vulkan_engine.render(&app);

        let mut done = false;
        events_loop.poll_events(|ev| match ev {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        info!("Close request!!");
                        done = true;
                    }
                    WindowEvent::Resized(_) => vulkan_engine.recreate_swapchain = true,
                    _ => {}
                }

                // Let app handle some events:
                app.handle_event(event);

                // Check for quit:
                if app.quit {
                    done = true;
                }
            }
            _ => (),
        });

        if done {
            info!("Leaving the GUI main loop");
            return;
        }

        let print_timing = false;
        if print_timing {
            let t2 = Instant::now();
            let duration = t2 - t1;
            let fps = 1.0 / (duration.as_micros() as f64 * 1.0e-6);
            println!("Duration of render loop: {:?} fps={}", duration, fps);
            t1 = t2;
        }
    }
}
