//! This module enables the rendering of a Chart struct.

use super::super::vertex::Vertex;
use crate::plot::{Chart, CurveData};
use cgmath::Matrix4;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, CpuBufferPool};
// use vulkano::buffer::immutable::ImmutableBuffer;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::Device;
use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};

type MyPipeline = Arc<dyn GraphicsPipelineAbstract + Send + Sync>;

/// A renderer engine specifically designed for charts.
pub struct ChartRenderer {
    device: Arc<Device>,
    uniform_buffer_pool: CpuBufferPool<vs::ty::Ubo1>,
    vertex_buffer_pool: CpuBufferPool<[Vertex; 100]>,
    pipeline: MyPipeline,
}

/*
fn from_sliced_data(data: &[Vertex]) -> [Vertex; 100] {
    let mut array: [Vertex; 100];
    let items = &data[..100];
    array.clone_from_slice(items);
    array
}
*/

impl ChartRenderer {
    pub fn new(
        device: Arc<Device>,
        render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    ) -> Self {
        let vs = vs::Shader::load(device.clone()).unwrap();
        let fs = fs::Shader::load(device.clone()).unwrap();

        let uniform_buffer_pool = CpuBufferPool::<vs::ty::Ubo1>::uniform_buffer(device.clone());
        let vertex_buffer_pool = CpuBufferPool::vertex_buffer(device.clone());

        let pipeline = Arc::new(
            GraphicsPipeline::start()
                .vertex_input(SingleBufferDefinition::<Vertex>::new())
                .vertex_shader(vs.main_entry_point(), ())
                .line_strip()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .render_pass(Subpass::from(render_pass, 0).unwrap())
                .build(device.clone())
                .unwrap(),
        );

        ChartRenderer {
            device,
            uniform_buffer_pool,
            vertex_buffer_pool,
            pipeline,
        }
    }

    pub fn draw(
        &self,
        mut started_renderer: AutoCommandBufferBuilder,
        dynamic_state: &mut DynamicState,
        chart: &Chart,
    ) -> AutoCommandBufferBuilder {
        for curve in &chart.curves {
            // println!("Rendering curve {:?}", curve);

            let uniform_buffer_subbuffer = {
                // Set tha zoom!
                let scaling = Matrix4::from_scale(0.01);
                let uniform_data = vs::ty::Ubo1 {
                    // dummy: Matrix4::one().into(),
                    scaling: scaling.into(),
                };

                self.uniform_buffer_pool.next(uniform_data).unwrap()
            };

            let set = Arc::new(
                PersistentDescriptorSet::start(self.pipeline.clone(), 0)
                    .add_buffer(uniform_buffer_subbuffer)
                    .unwrap()
                    .build()
                    .unwrap(),
            );

            let vertex_buffer = {
                if let CurveData::Trace(trace) = &curve.data {
                    let inner_data: Vec<Vertex> = trace
                        .to_vec()
                        .iter()
                        .map(|sample| {
                            let x = sample.timestamp.amount as f32;
                            let y = sample.value as f32;
                            Vertex { position: [x, y] }
                        })
                        .collect();

                    /*
                    let test_data: Vec<Vertex> = (1..100).map(|v| {
                        let x = (v as f32) * 0.005_f32;
                        let y = x;
                        Vertex {
                            position: [x, y],
                        }

                    }).collect();
                    */
                    // device

                    CpuAccessibleBuffer::from_iter(
                        self.device.clone(),
                        BufferUsage::vertex_buffer(),
                        inner_data.iter().cloned(),
                    )
                    .unwrap()
                // let v: [Vertex; 100] = from_sliced_data(&test_data);
                // ImmutableBuffer::vert
                // Arc::new(self.vertex_buffer_pool.next(v).unwrap())
                } else {
                    unimplemented!("Curve data not implemented");
                }
            };

            // let (vertex_subbuffer, handle1) = res1;

            started_renderer = started_renderer
                .draw(
                    self.pipeline.clone(),
                    dynamic_state,
                    vec![vertex_buffer],
                    set,
                    (),
                )
                .unwrap();
        }

        started_renderer
    }
}

// Shaders for graph:

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
    gl_Position = ubo.scaling * vec4(position, 0.0, 1.0);  // <-- works!
}
"
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
