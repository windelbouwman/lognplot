//! Vulkan rendering engine.

use super::text::TextEngine;
use std::cell::RefCell;
// use super::visuals::ChartRenderer;
use std::sync::Arc;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::{Device, Queue};
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract};
use vulkano::image::SwapchainImage;
use vulkano::instance::Instance;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{self, AcquireError, PresentMode, SurfaceTransform, Swapchain};
// use vulkano::
use vulkano::swapchain::Surface;
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};
use winit::Window;

use super::super::super::MainApp;
use super::instance::{create_device_and_queue, create_render_pass};

pub struct VulkanEngine {
    pub text_engine: TextEngine,
    // chart_engine: ChartRenderer,
    draw_queue: Vec<DrawThing>,

    // Internal vulkan state:
    surface: Arc<Surface<Window>>,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: DynamicState,
    framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,

    // TODO: refactor flow of this bool (remove the boolean)
    pub recreate_swapchain: bool,

    previous_frame_end: RefCell<Option<Box<dyn GpuFuture>>>,
}

enum DrawThing {
    Text { x: f32, y: f32, text: String },
    // Chart,
}

impl VulkanEngine {
    pub fn new(instance: Arc<Instance>, surface: Arc<Surface<Window>>) -> Self {
        // surface.window();

        // Create device and queue onto window:
        let (device, queue) = create_device_and_queue(&instance, surface.clone());
        let (swapchain, images) = create_swap_chain(device.clone(), queue.clone(), surface.clone());
        let render_pass = create_render_pass(device.clone(), swapchain.clone());

        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
        };

        let framebuffers =
            window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);

        let text_engine = TextEngine::new(device.clone(), render_pass.clone());
        let previous_frame_end = RefCell::new(Some(
            Box::new(sync::now(device.clone())) as Box<dyn GpuFuture>
        ));

        Self {
            text_engine,
            draw_queue: vec![],

            // Vulkano internals:
            surface,
            device,
            queue,
            swapchain,
            images: vec![],
            render_pass,
            dynamic_state,
            framebuffers,
            recreate_swapchain: false,
            previous_frame_end,
        }
    }

    pub fn begin_draw(&mut self) {
        self.draw_queue.clear();
    }

    // 2D API:
    pub fn draw_text(&self, x: f32, y: f32, text: &str) {
        self.text_engine.queue_text(x, y, text);
        // self.draw_queue.push(DrawThing::Text {
        //     x,
        //     y,
        //     text: text.to_string(),
        // });
    }

    // TODO:
    // - draw line
    // - draw rectangle
    // - apply scaling

    // Rendering api:
    pub fn render(&mut self, app: &MainApp) {
        // Start rendering:
        self.begin_draw();

        // Draw the app in the vulkan engine:
        app.paint(self);

        self.inner_render();
    }

    fn inner_render(&mut self) {
        let mut prev_frame = self.previous_frame_end.replace(None).unwrap();
        prev_frame.cleanup_finished();

        if self.recreate_swapchain {
            // Get the new dimensions of the window.
            let dimensions = if let Some(dimensions) = self.surface.window().get_inner_size() {
                let dimensions: (u32, u32) = dimensions
                    .to_physical(self.surface.window().get_hidpi_factor())
                    .into();
                [dimensions.0, dimensions.1]
            } else {
                return;
            };

            self.recreate_swap_chain(dimensions);
            self.recreate_swapchain = false;
        }

        let (image_num, acquire_future) =
            match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    self.previous_frame_end
                        .replace(Some(Box::new(sync::now(self.device.clone()))));
                    self.recreate_swapchain = true;
                    return;
                }
                Err(err) => panic!("{:?}", err),
            };

        let queue_family = self.queue.family();
        let mut command_buffer =
            AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), queue_family)
                .unwrap();

        // Command buffer elements before rendering:
        command_buffer = self
            .text_engine
            .prepare_buffers(command_buffer, queue_family);

        // Specify the color to clear the framebuffer with i.e. blue
        let clear_values = vec![[0.0, 0.0, 1.0, 1.0].into()];

        let mut started_renderer = command_buffer
            .begin_render_pass(self.framebuffers[image_num].clone(), false, clear_values)
            .unwrap();

        // We are now inside the first subpass of the render pass. We add a draw command.
        started_renderer = self.draw(started_renderer);

        let command_buffer = started_renderer.end_render_pass().unwrap().build().unwrap();

        let future = prev_frame
            .join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            // The color output is now expected to contain our triangle. But in order to show it on
            // the screen, we have to *present* the image by calling `present`.
            //
            // This function does not actually present the image immediately. Instead it submits a
            // present command at the end of the queue. This means that it will only be presented once
            // the GPU has finished executing the command buffer that draws the triangle.
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
            .then_signal_fence_and_flush();

        let new_fut = match future {
            Ok(future) => Box::new(future) as Box<dyn GpuFuture>,
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                Box::new(sync::now(self.device.clone()))
            }
            Err(e) => {
                println!("{:?}", e);
                Box::new(sync::now(self.device.clone()))
            }
        };

        self.previous_frame_end.replace(Some(new_fut));
    }

    pub fn recreate_swap_chain(&mut self, dimensions: [u32; 2]) {
        let (new_swapchain, new_images) = match self.swapchain.recreate_with_dimension(dimensions) {
            Ok(r) => r,
            // This error tends to happen when the user is manually resizing the window.
            // Simply restarting the loop is the easiest way to fix this issue.
            // Err(SwapchainCreationError::UnsupportedDimensions) => continue,
            Err(err) => panic!("swapchain recreation error: {:?}", err),
        };

        self.swapchain = new_swapchain;
        // Because framebuffers contains an Arc on the old swapchain, we need to
        // recreate framebuffers as well.
        self.framebuffers = window_size_dependent_setup(
            &new_images,
            self.render_pass.clone(),
            &mut self.dynamic_state,
        );
    }

    // Emit draw commands
    fn draw(&mut self, mut started_renderer: AutoCommandBufferBuilder) -> AutoCommandBufferBuilder {
        started_renderer = self
            .text_engine
            .emit_draw_calls(started_renderer, &mut self.dynamic_state);
        started_renderer
    }
}

fn create_swap_chain(
    device: Arc<Device>,
    queue: Arc<Queue>,
    surface: Arc<Surface<Window>>,
) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
    // Querying the capabilities of the surface. When we create the swapchain we can only
    // pass values that are allowed by the capabilities.
    let physical = device.physical_device();
    let caps = surface.capabilities(physical).unwrap();

    let usage = caps.supported_usage_flags;

    // The alpha mode indicates how the alpha value of the final image will behave. For example
    // you can choose whether the window will be opaque or transparent.
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();

    // Choosing the internal format that the images will have.
    let format = caps.supported_formats[0].0;

    // The dimensions of the window, only used to initially setup the swapchain.
    // NOTE:
    // On some drivers the swapchain dimensions are specified by `caps.current_extent` and the
    // swapchain size must use these dimensions.
    // These dimensions are always the same as the window dimensions
    //
    // However other drivers dont specify a value i.e. `caps.current_extent` is `None`
    // These drivers will allow anything but the only sensible value is the window dimensions.
    //
    // Because for both of these cases, the swapchain needs to be the window dimensions, we just use that.
    let window = surface.window();
    let initial_dimensions = if let Some(dimensions) = window.get_inner_size() {
        // convert to physical pixels
        let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
        [dimensions.0, dimensions.1]
    } else {
        // The window no longer exists so exit the application.
        // return;
        unimplemented!("Quit app from here!");
    };

    // Please take a look at the docs for the meaning of the parameters we didn't mention.
    Swapchain::new(
        device.clone(),
        surface.clone(),
        caps.min_image_count,
        format,
        initial_dimensions,
        1,
        usage,
        &queue,
        SurfaceTransform::Identity,
        alpha,
        PresentMode::Fifo,
        true,
        None,
    )
    .unwrap()
}

/// This method is called once during initialization, then again whenever the window is resized
fn window_size_dependent_setup(
    images: &Vec<Arc<SwapchainImage<Window>>>,
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}
