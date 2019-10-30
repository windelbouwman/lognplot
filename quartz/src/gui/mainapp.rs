use winit::VirtualKeyCode;
// TODO: turn into interface?
use super::backends::vulkan::VulkanEngine;
use super::backends::Paintable;

use super::GraphControl;
use crate::plot::plot;
use crate::plot::{Chart, Curve, CurveData};
use quartzcanvas::Canvas;
use quartzgui::widgets::{Button, Container};
use quartztsdb::TsDbHandle;

/// Application structure.
pub struct MainApp {
    zoom_in: bool,
    zoom_out: bool,
    zoom: f32,
    horizontal_scroll: i32,
    pub quit: bool,
    db: TsDbHandle,
    chart: Chart,
    root_container: Container,
}

fn test1(canvas: &mut dyn Canvas) {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0, 8.0];
    let y = vec![9.0, 2.2, 5.5, 2.2, 1.2, 1.7];

    plot(canvas, x, y);
}

impl Paintable for MainApp {
    /// Paint the application onto some rendering surface!
    fn paint(&self, engine: &mut VulkanEngine) {
        engine.draw_text(100.0, -350.0, &format!("FPS={:.2}", engine.fps()));

        engine.draw_text(
            0.0,
            0.0,
            &format!("hoi: zoom={} pan ={}", self.zoom, self.horizontal_scroll),
        );

        engine.draw_text(
            0.0,
            -150.0,
            &format!("Horizontal pan={}", self.horizontal_scroll),
        );

        engine.draw_text(0.1, 130.0, "boe ba beloeba!");

        let trc = self.db.get_trace("Trace0");
        let ln = trc.len();
        engine.draw_text(-80.0, -100.0, &format!("Trace {}!", ln));

        test1(engine);
    }
}

impl MainApp {
    pub fn new(db: TsDbHandle) -> Self {
        let trace = db.get_trace("Trace0");
        let curve1 = Curve::new(CurveData::Trace(trace));
        let mut chart = Chart::default();
        chart.set_title("Plot1");
        chart.add_curve(curve1);

        let mut root_container = Container::new();
        let b1 = Button::new();
        root_container.add_child(b1);
        let b2 = Button::new();
        root_container.add_child(b2);
        let g1 = GraphControl::new(chart.clone());
        root_container.add_child(g1);

        MainApp {
            zoom_in: false,
            zoom_out: false,
            zoom: 1.0_f32,
            horizontal_scroll: 0,
            quit: false,
            db,
            chart,
            root_container,
        }
    }

    /// Update loop of the application
    pub fn tick(&mut self) {
        if self.zoom_in {
            self.zoom *= 1.05_f32;
        }

        if self.zoom_out {
            self.zoom *= 0.95_f32;
        }
    }

    pub fn handle_event(&mut self, event: winit::WindowEvent) {
        match event {
            winit::WindowEvent::KeyboardInput { input, .. } => match input.state {
                winit::ElementState::Pressed => {
                    if let Some(virtual_keycode) = input.virtual_keycode {
                        self.handle_key_press(virtual_keycode)
                    }
                }
                winit::ElementState::Released => {
                    if let Some(virtual_keycode) = input.virtual_keycode {
                        self.handle_key_release(virtual_keycode);
                    }
                }
            },
            _ => {}
        }
    }

    fn handle_key_press(&mut self, virtual_keycode: VirtualKeyCode) {
        match virtual_keycode {
            VirtualKeyCode::D => {
                // info!("Zoom out pressed");
                self.zoom_out = true;
            }
            VirtualKeyCode::S => {
                // info!("Zoom in");
                self.zoom_in = true;
            }
            VirtualKeyCode::Left => {
                self.horizontal_scroll = -1;
            }
            VirtualKeyCode::Right => {
                self.horizontal_scroll = 1;
            }
            VirtualKeyCode::Q | VirtualKeyCode::Escape => {
                self.quit = true;
            }
            keycode => {
                info!("KEY {:?}", keycode);
            }
        }
    }

    fn handle_key_release(&mut self, virtual_keycode: VirtualKeyCode) {
        match virtual_keycode {
            VirtualKeyCode::D => {
                // info!("Zoom out");
                self.zoom_out = false;
            }
            VirtualKeyCode::S => {
                // info!("Zoom in released");
                self.zoom_in = false;
            }
            VirtualKeyCode::Left => {
                self.horizontal_scroll = 0;
            }
            VirtualKeyCode::Right => {
                self.horizontal_scroll = 0;
            }
            _ => {}
        }
    }
}
