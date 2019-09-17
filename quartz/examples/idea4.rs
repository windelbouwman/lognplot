use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;

use vulkano::instance::PhysicalDevice;

use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;

extern crate winit;

fn main() {
    simple_logger::init().unwrap();

    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
    };

    println!("Woot! Got instance!");

    for p in PhysicalDevice::enumerate(&instance) {
        println!("Got physical device: {:?}", p);
    }

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    for family in physical.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    let (device, mut queues) = {
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            ..vulkano::device::DeviceExtensions::none()
        };

        Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };

    let queue = queues.next().unwrap();

    println!("Got queue!!! {:?}", queue);

    let data = 12;
    let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), data)
        .expect("failed to create buffer");

    let iter = (0..128).map(|_| 5u8);
    let buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), iter).unwrap();

    let source_content = 0..64;
    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), source_content)
        .expect("failed to create buffer");

    let dest_content = (0..64).map(|_| 0);
    let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), dest_content)
        .expect("failed to create buffer");

    use vulkano::command_buffer::AutoCommandBufferBuilder;

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family())
        .unwrap()
        .copy_buffer(source.clone(), dest.clone())
        .unwrap()
        .build()
        .unwrap();

    // println!("Command buffer!! {:?}", command_buffer);

    use vulkano::command_buffer::CommandBuffer;
    let finished = command_buffer.execute(queue.clone()).unwrap();

    use vulkano::sync::GpuFuture;

    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let data_iter = 0..65536;
    let data_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), data_iter)
        .expect("failed to create buffer");

    mod cs {
        vulkano_shaders::shader! {
            ty: "compute",
            src: "
    #version 450

    layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

    layout(set = 0, binding = 0) buffer Data {
        uint data[];
    } buf;

    void main() {
        uint idx = gl_GlobalInvocationID.x;
        buf.data[idx] *= 12;
    }"
        }
    }

    let shader = cs::Shader::load(device.clone()).expect("failed to create shader module");

    use std::sync::Arc;
    use vulkano::pipeline::ComputePipeline;

    let compute_pipeline = Arc::new(
        ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
            .expect("failed to create compute pipeline"),
    );

    use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;

    let set = Arc::new(
        PersistentDescriptorSet::start(compute_pipeline.clone(), 0)
            .add_buffer(data_buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family())
        .unwrap()
        .dispatch([1024, 1, 1], compute_pipeline.clone(), set.clone(), ())
        .unwrap()
        .build()
        .unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("Everything succeeded!");

    let mut events_loop = winit::EventsLoop::new();

    use vulkano_win::VkSurfaceBuild;
    use winit::EventsLoop;
    use winit::WindowBuilder;

    let surface = WindowBuilder::new()
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();

    let caps = surface
        .capabilities(physical)
        .expect("failed to get surface capabilities");

    let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
    let format = caps.supported_formats[0].0;

    use vulkano::swapchain::{PresentMode, SurfaceTransform, Swapchain};

    let (swapchain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        caps.min_image_count,
        format,
        dimensions,
        1,
        caps.supported_usage_flags,
        &queue,
        SurfaceTransform::Identity,
        alpha,
        PresentMode::Fifo,
        true,
        None,
    )
    .expect("failed to create swapchain");

    let (image_num, acquire_future) =
        vulkano::swapchain::acquire_next_image(swapchain.clone(), None).unwrap();

    events_loop.run_forever(|event| match event {
        winit::Event::WindowEvent {
            event: winit::WindowEvent::CloseRequested,
            ..
        } => winit::ControlFlow::Break,
        _ => winit::ControlFlow::Continue,
    });
}
