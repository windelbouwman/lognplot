use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, CpuBufferPool};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::device::Device;
use vulkano::framebuffer::{RenderPassAbstract, RenderPassDesc, Subpass};
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};

use cgmath::Matrix4;

use std::sync::Arc;

use super::vertex::Vertex;

type MyPipeline = Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

pub struct MyVisual {
    uniform_buffer: CpuBufferPool<vs::ty::Ubo1>,
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    pipeline: MyPipeline,
    pub zoom: f32, // How much to zoom this line?
    trace: f32,
}

impl MyVisual {
    pub fn new(
        device: Arc<Device>,
        render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
        amp: f32,
    ) -> Self {
        // We now create a buffer that will store the shape of our triangle.
        let vertex_buffer = {
            let mut points: Vec<Vertex> = vec![];
            for t in -10000..10000 {
                let x = (t as f32) * 0.0001; // to seconds
                let y = amp * (x * 3.14159 * 2.0 * 1.0).sin()
                    + 0.03 * (x * 3.14159 * 2.0 * 200.0).sin();
                points.push(Vertex { position: [x, y] });
            }

            CpuAccessibleBuffer::from_iter(
                device.clone(),
                BufferUsage::all(),
                points.iter().cloned(),
            )
            .unwrap()
        };

        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();

        let uniform_buffer = CpuBufferPool::<vs::ty::Ubo1>::uniform_buffer(device.clone());

        let pipeline = Arc::new(
            GraphicsPipeline::start()
                // We need to indicate the layout of the vertices.
                // The type `SingleBufferDefinition` actually contains a template parameter corresponding
                // to the type of each vertex. But in this code it is automatically inferred.
                .vertex_input(SingleBufferDefinition::<Vertex>::new())
                // .vertex_input_single_buffer()
                // A Vulkan shader can in theory contain multiple entry points, so we have to specify
                // which one. The `main` word of `main_entry_point` actually corresponds to the name of
                // the entry point.
                .vertex_shader(vs.main_entry_point(), ())
                // The content of the vertex buffer describes a list of triangles.
                // .triangle_list()
                // .lines()
                // .point_list()
                .line_strip()
                // Use a resizable viewport set to draw over the entire window
                .viewports_dynamic_scissors_irrelevant(1)
                // See `vertex_shader`.
                .fragment_shader(fs.main_entry_point(), ())
                // We have to indicate which subpass of which render pass this pipeline is going to be used
                // in. The pipeline will only be usable from this particular subpass.
                .render_pass(Subpass::from(render_pass, 0).unwrap())
                // Now that our builder is filled, we call `build()` to obtain an actual pipeline.
                .build(device.clone())
                .unwrap(),
        );

        MyVisual {
            uniform_buffer,
            vertex_buffer,
            pipeline,
            zoom: 1.0_f32,
            trace: 3.14_f32,
        }
    }

    pub fn draw(
        &self,
        started_renderer: AutoCommandBufferBuilder,
        dynamic_state: &mut DynamicState,
    ) -> AutoCommandBufferBuilder {
        let uniform_buffer_subbuffer = {
            // Set tha zoom!
            // let mut scaling = Matrix2::one();
            // scaling[0][0] = self.zoom;
            // Somehow, the mat2 does not work?
            // let scaling = Matrix2::new(self.zoom, 0.0_f32, 0.0_f32, 0.9_f32);
            let mut scaling = Matrix4::from_scale(1.0);
            scaling[0][0] = self.zoom;
            // println!("Scaling matrix: {:?}", scaling);

            let uniform_data = vs::ty::Ubo1 {
                // dummy: Matrix4::one().into(),
                scaling: scaling.into(),
            };

            self.uniform_buffer.next(uniform_data).unwrap()
        };

        let set = Arc::new(
            PersistentDescriptorSet::start(self.pipeline.clone(), 0)
                .add_buffer(uniform_buffer_subbuffer)
                .unwrap()
                .build()
                .unwrap(),
        );

        let in_progress_renderer = started_renderer
            .draw(
                self.pipeline.clone(),
                dynamic_state,
                vec![self.vertex_buffer.clone()],
                set,
                (),
            )
            .unwrap();
        in_progress_renderer
    }
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
#version 450

layout(location = 0) in vec2 position;

// Add some cool scaling factor!!
layout(set = 0, binding = 0) uniform Ubo1 {
    mat4 scaling;
} ubo;

void main() {
    // vec2 p = ubo.scaling * position;
    // gl_Position = vec4(p, 0.0, 1.0);  // <-- does not work??
    gl_Position = ubo.scaling * vec4(position, 0.0, 1.0);  // <-- works!
}"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
#version 450

layout(location = 0) out vec4 f_color;

void main() {
f_color = vec4(1.0, 0.0, 0.0, 1.0);
}
"
    }
}
