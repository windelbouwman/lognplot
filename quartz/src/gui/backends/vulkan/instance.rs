use std::sync::Arc;
use vulkano::instance::{self, Instance, PhysicalDevice};

use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::swapchain::Swapchain;

use vulkano::swapchain::Surface;
use winit::Window;

pub fn create_vulkan_instance() -> Arc<Instance> {
    println!("List of Vulkan debugging layers available to use:");
    let mut layers = instance::layers_list().unwrap();
    while let Some(l) = layers.next() {
        println!("\t{}", l.name());
    }

    // Select the validation layers we want to use:
    // let layer = "VK_LAYER_LUNARG_standard_validation";
    let layers = vec![];

    let instance = {
        let mut extensions = vulkano_win::required_extensions();
        extensions.ext_debug_report = true;
        Instance::new(None, &extensions, layers).unwrap()
    };
    instance
}

/// Given an vulkano instance, create a vulkan device and a queue.
pub fn create_device_and_queue(
    instance: &Arc<Instance>,
    surface: Arc<Surface<Window>>,
) -> (Arc<Device>, Arc<Queue>) {
    let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
    info!(
        "Using device: {} (type: {:?})",
        physical.name(),
        physical.ty()
    );

    let queue_family = physical
        .queue_families()
        .find(|&q| {
            // We take the first queue that supports drawing to our window.
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        })
        .unwrap();

    // The list of created queues is returned by the function alongside with the device.
    let device_ext = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };
    let (device, mut queues) = Device::new(
        physical,
        physical.supported_features(),
        &device_ext,
        [(queue_family, 0.5)].iter().cloned(),
    )
    .unwrap();

    let queue = queues.next().unwrap();

    (device, queue)
}

pub fn create_render_pass(
    device: Arc<Device>,
    swapchain: Arc<Swapchain<Window>>,
) -> Arc<dyn RenderPassAbstract + Send + Sync> {
    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                // `color` is a custom name we give to the first and only attachment.
                color: {
                    // `load: Clear` means that we ask the GPU to clear the content of this
                    // attachment at the start of the drawing.
                    load: Clear,
                    // `store: Store` means that we ask the GPU to store the output of the draw
                    // in the actual image. We could also ask it to discard the result.
                    store: Store,
                    // `format: <ty>` indicates the type of the format of the image. This has to
                    // be one of the types of the `vulkano::format` module (or alternatively one
                    // of your structs that implements the `FormatDesc` trait). Here we use the
                    // same format as the swapchain.
                    format: swapchain.format(),
                    // TODO:
                    samples: 1,
                }
            },
            pass: {
                // We use the attachment named `color` as the one and only color attachment.
                color: [color],
                // No depth-stencil attachment is indicated with empty brackets.
                depth_stencil: {}
            }
        )
        .unwrap(),
    );

    render_pass
}
