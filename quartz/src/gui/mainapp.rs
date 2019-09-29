use std::sync::Arc;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::Device;
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::instance::QueueFamily;
use winit::VirtualKeyCode;

use super::text::TextEngine;
use super::visual1::MyVisual;
use super::visuals::ChartRenderer;
use crate::plot::{Chart, Curve, CurveData};
use crate::tsdb::TsDbHandle;

/// Application structure.
pub struct MainApp {
    device: Arc<Device>,
    visuals: Vec<MyVisual>,
    text_engine: TextEngine,
    chart_engine: ChartRenderer,
    zoom_in: bool,
    zoom_out: bool,
    horizontal_scroll: i32,
    pub quit: bool,
    db: TsDbHandle,
    chart: Chart,
}

impl MainApp {
    pub fn new(
        device: Arc<Device>,
        render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
        db: TsDbHandle,
    ) -> Self {
        let my_visual = MyVisual::new(device.clone(), render_pass.clone(), 0.7_f32);
        let my_visual1 = MyVisual::new(device.clone(), render_pass.clone(), 0.3_f32);

        // array with visuals:
        let visuals = vec![my_visual, my_visual1];

        let text_engine = TextEngine::new(device.clone(), render_pass.clone());
        let chart_engine = ChartRenderer::new(device.clone(), render_pass.clone());

        let trace = db.get_trace("Trace0");
        let curve1 = Curve::new(CurveData::Trace(trace));
        let mut chart = Chart::default();
        chart.set_title("Plot1");
        chart.add_curve(curve1);

        MainApp {
            device,
            visuals,
            text_engine,
            chart_engine,
            zoom_in: false,
            zoom_out: false,
            horizontal_scroll: 0,
            quit: false,
            db,
            chart,
        }
    }

    /// Update loop of the application
    pub fn tick(&mut self) {
        if self.zoom_in {
            self.visuals[0].zoom *= 1.05_f32;
        }

        if self.zoom_out {
            self.visuals[0].zoom *= 0.95_f32;
        }

        self.visuals[0].pan += (self.horizontal_scroll as f32) * 0.05;

        self.text_engine.queue_text(
            0.0,
            0.0,
            &format!(
                "hoi: zoom={} pan ={}",
                self.visuals[0].zoom, self.visuals[0].pan
            ),
        );

        self.text_engine.queue_text(
            0.0,
            -150.0,
            &format!("Horizontal pan={}", self.horizontal_scroll),
        );

        self.text_engine.queue_text(0.1, 130.0, "boe ba beloeba!");

        let trc = self.db.get_trace("Trace0");
        let ln = trc.len();
        self.text_engine
            .queue_text(-80.0, -100.0, &format!("Trace {}!", ln));
    }

    pub fn prepare_commands(
        &self,
        command_buffer: AutoCommandBufferBuilder,
        queue_family: QueueFamily,
    ) -> AutoCommandBufferBuilder {
        self.text_engine
            .prepare_buffers(command_buffer, queue_family)
    }

    pub fn draw(
        &self,
        mut started_renderer: AutoCommandBufferBuilder,
        dynamic_state: &mut DynamicState,
    ) -> AutoCommandBufferBuilder {
        for visual in self.visuals.iter() {
            started_renderer = visual.draw(started_renderer, dynamic_state);
        }

        // Invoke the charting engine on the different charts!
        started_renderer = self
            .chart_engine
            .draw(started_renderer, dynamic_state, &self.chart);

        started_renderer = self
            .text_engine
            .emit_draw_calls(started_renderer, dynamic_state);

        started_renderer
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
