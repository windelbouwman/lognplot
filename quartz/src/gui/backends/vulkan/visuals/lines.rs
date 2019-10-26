//! Line drawing using lyon.
//!
//! This engine will take lyon paths, and render them to the screen.
//! Essentially this wires lyon onto vulkan in some weird way.
//!
//! Ideally this would involve some caching.

use std::sync::Arc;

use cgmath::Matrix4;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, CpuBufferPool};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::Device;
use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};

use lyon::math::point;
use lyon::path::Path;
use lyon::tessellation::{
    geometry_builder::simple_builder, StrokeOptions, StrokeTessellator, StrokeVertex, VertexBuffers,
};

use super::super::vertex::Vertex;

pub struct LyonEngine {
    // cache: todo
    vertex_buffer: Arc<CpuAccessibleBuffer<[Vertex]>>,
    index_buffer: Arc<CpuAccessibleBuffer<[u16]>>,
    uniform_buffer: CpuBufferPool<vs::ty::Ubo1>,
    pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
}

impl LyonEngine {
    pub fn new(
        device: Arc<Device>,
        render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    ) -> Self {
        ////// DEMO PURPOSE!
        // Create a simple path.
        let mut path_builder = Path::builder();
        path_builder.move_to(point(0.0, 0.0));
        path_builder.line_to(point(100.0, 200.0));
        path_builder.line_to(point(200.0, 0.0));
        path_builder.line_to(point(100.0, 100.0));
        // path_builder.close();
        let path = path_builder.build();

        let mut buffers: VertexBuffers<StrokeVertex, u16> = VertexBuffers::new();
        {
            // Create the destination vertex and index buffers.
            let mut vertex_builder = simple_builder(&mut buffers);

            // Create the tessellator.
            let mut tessellator = StrokeTessellator::new();

            let stroke_options = StrokeOptions::default().with_line_width(7.0);
            // Compute the tessellation.
            tessellator
                .tessellate_path(&path, &stroke_options, &mut vertex_builder)
                .unwrap();
        }

        // println!("The generated vertices are: {:?}.", &buffers.vertices[..]);
        // println!("The generated indices are: {:?}.", &buffers.indices[..]);
        // self.queue.push(buffers.vertices[..], buffers.indices[..]);

        let scale = 0.005;
        let vertex_buffer = {
            let mut points: Vec<Vertex> = vec![];
            for l_v in &buffers.vertices {
                points.push(Vertex {
                    position: [l_v.position.x * scale, l_v.position.y * scale],
                });
            }

            CpuAccessibleBuffer::from_iter(
                device.clone(),
                BufferUsage::vertex_buffer(),
                points.iter().cloned(),
            )
            .unwrap()
        };

        let index_buffer = {
            let mut indices: Vec<u16> = vec![];
            for l_i in &buffers.indices {
                indices.push(*l_i);
            }

            CpuAccessibleBuffer::from_iter(
                device.clone(),
                BufferUsage::index_buffer(),
                indices.iter().cloned(),
            )
            .unwrap()
        };

        let uniform_buffer = CpuBufferPool::<vs::ty::Ubo1>::uniform_buffer(device.clone());

        // Load proper shaders:
        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();

        // Create the pipeline we will use:
        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input(SingleBufferDefinition::<Vertex>::new())
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .render_pass(Subpass::from(render_pass, 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        LyonEngine {
            index_buffer,
            vertex_buffer,
            uniform_buffer,
            pipeline,
        }
    }

    /// Call this function to draw a line thingy!
    pub fn make_line(&mut self) {
        // Create a simple path.
        let mut path_builder = Path::builder();
        path_builder.move_to(point(0.0, 0.0));
        path_builder.line_to(point(100.0, 200.0));
        path_builder.line_to(point(200.0, 0.0));
        path_builder.line_to(point(100.0, 100.0));
        path_builder.close();
        let path = path_builder.build();

        self.stroke_path(path);
    }

    /// Call this function to stroke a given path.
    pub fn stroke_path(&mut self, path: Path) {
        // Create the destination vertex and index buffers.
        let mut buffers: VertexBuffers<StrokeVertex, u16> = VertexBuffers::new();
        {
            // Create the destination vertex and index buffers.
            let mut vertex_builder = simple_builder(&mut buffers);

            // Create the tessellator.
            let mut tessellator = StrokeTessellator::new();

            // Compute the tessellation.
            tessellator
                .tessellate_path(&path, &StrokeOptions::default(), &mut vertex_builder)
                .unwrap();
        }

        // println!("The generated vertices are: {:?}.", &buffers.vertices[..]);
        // println!("The generated indices are: {:?}.", &buffers.indices[..]);
        // self.queue.push(buffers.vertices[..], buffers.indices[..]);
    }

    pub fn draw(
        &self,
        command_buffer_builder: AutoCommandBufferBuilder,
        dynamic_state: &mut DynamicState,
    ) -> AutoCommandBufferBuilder {
        let uniform_buffer_subbuffer = {
            // Set tha zoom!
            // let mut scaling = Matrix2::one();
            // scaling[0][0] = self.zoom;
            // Somehow, the mat2 does not work?
            // let scaling = Matrix2::new(self.zoom, 0.0_f32, 0.0_f32, 0.9_f32);
            let scaling = Matrix4::from_scale(1.0);
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

        command_buffer_builder
            .draw_indexed(
                self.pipeline.clone(),
                dynamic_state,
                vec![self.vertex_buffer.clone()],
                self.index_buffer.clone(),
                set,
                (),
            )
            .unwrap()
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
