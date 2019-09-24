use rusttype::gpu_cache;
use rusttype::{point, Font, PositionedGlyph, Rect, Scale};
// use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::Arc;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::Device;
use vulkano::format::R8Unorm;
use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use vulkano::image::{Dimensions, ImageLayout, ImageUsage, ImmutableImage};
use vulkano::instance::QueueFamily;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode};

/// Text drawing for vulkan
///
/// Text is a problematic issue in opengl. There
/// are several ideas. The most basic is to create a texture
/// with glyphs.
///
/// We will take the approach of a cache of glyphs with a
/// texture handle per glyph.
/// When text is drawn, new glyphs are created on the fly.

// Idea/copied from here: https://github.com/rukai/vulkano-text/blob/master/src/lib.rs

const CACHE_WIDTH: usize = 1024;
const CACHE_HEIGHT: usize = 1024;

// struct MyGlyph {
// texture_buffer: Arc<Buffer>,
// }

/// Text engine, holding cached glyphs as textures.
/// Capable of drawing new texts.
pub struct TextEngine {
    // cache: HashMap<char, Arc<MyGlyph>>,
    device: Arc<Device>,
    cache: RefCell<gpu_cache::Cache<'static>>,
    cache_pixel_buffer: RefCell<Vec<u8>>,
    font: Font<'static>,
    pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,
    sampler: Arc<Sampler>,
    glyph_queue: RefCell<Vec<PositionedGlyph<'static>>>,
    update_required: RefCell<bool>,
    cache_texture: RefCell<Option<Arc<ImmutableImage<R8Unorm>>>>,
}

impl TextEngine {
    pub fn new(
        device: Arc<Device>,
        render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    ) -> Self {
        // Font downloaded from:
        // https://fontlibrary.org/en/font/archicoco
        let font_bytes = include_bytes!("Archicoco.ttf");
        let font = Font::from_bytes(font_bytes as &[u8]).expect("Font must be valid ttf data!");

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

        // TODO: move this sampler
        let sampler = Sampler::new(
            device.clone(),
            Filter::Linear,
            Filter::Linear,
            MipmapMode::Nearest,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            SamplerAddressMode::Repeat,
            0.0,
            1.0,
            0.0,
            0.0,
        )
        .unwrap();

        let cache = RefCell::new(
            gpu_cache::Cache::builder()
                .dimensions(CACHE_WIDTH as u32, CACHE_HEIGHT as u32)
                .build(),
        );
        let cache_pixel_buffer = RefCell::new(vec![0; CACHE_WIDTH * CACHE_HEIGHT]);

        TextEngine {
            device,
            cache,
            cache_pixel_buffer,
            font,
            pipeline,
            sampler,
            glyph_queue: RefCell::new(vec![]),
            update_required: RefCell::new(true),
            cache_texture: RefCell::new(None),
        }
    }

    pub fn queue_text(&self, x: f32, y: f32, text: &str) {
        // TODO: a draw call might return a commandbuffer?
        // Or put the command buffer in the queue?

        // Phase 1: draw the text, and create positioned glyphs:
        let size = 72.0;
        let glyphs: Vec<PositionedGlyph> = self
            .font
            .layout(text, Scale::uniform(size), point(x, y))
            .map(|x| x.standalone())
            .collect();

        for glyph in &glyphs {
            self.cache.borrow_mut().queue_glyph(0, glyph.clone());
        }

        self.glyph_queue.borrow_mut().extend(glyphs);
    }

    pub fn prepare_buffers(
        &self,
        mut command_buffer: AutoCommandBufferBuilder,
        queue_family: QueueFamily,
    ) -> AutoCommandBufferBuilder {
        self.flush_cache();

        if *self.update_required.borrow() {
            self.update_required.replace(false);

            let buffer = CpuAccessibleBuffer::<[u8]>::from_iter(
                self.device.clone(),
                BufferUsage::all(),
                self.cache_pixel_buffer.borrow().iter().cloned(),
            )
            .unwrap();

            let (cache_texture, cache_texture_write) = ImmutableImage::uninitialized(
                self.device.clone(),
                Dimensions::Dim2d {
                    width: CACHE_WIDTH as u32,
                    height: CACHE_HEIGHT as u32,
                },
                R8Unorm,
                1,
                ImageUsage {
                    sampled: true,
                    transfer_destination: true,
                    ..ImageUsage::none()
                },
                ImageLayout::General,
                Some(queue_family),
            )
            .unwrap();

            command_buffer = command_buffer
                .copy_buffer_to_image(buffer.clone(), cache_texture_write)
                .unwrap();

            self.cache_texture.replace(Some(cache_texture));
        }
        command_buffer
    }

    /// Flush out any queued draw calls
    pub fn emit_draw_calls(
        &self,
        started_renderer: AutoCommandBufferBuilder,
        dynamic_state: &mut DynamicState,
    ) -> AutoCommandBufferBuilder {
        let cache_texture = self
            .cache_texture
            .borrow()
            .clone()
            .expect("Cached texture must be present!");

        let set = Arc::new(
            PersistentDescriptorSet::start(self.pipeline.clone(), 0)
                .add_sampled_image(cache_texture.clone(), self.sampler.clone())
                .unwrap()
                .build()
                .unwrap(),
        );

        let vertices: Vec<Vertex> = self
            .glyph_queue
            .borrow_mut()
            .drain(..)
            .flat_map(|glyph| self.get_glyph_vertices(&glyph))
            .collect();

        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(),
            BufferUsage::all(),
            vertices.into_iter(),
        )
        .unwrap();

        started_renderer
            .draw(
                self.pipeline.clone(),
                dynamic_state,
                vec![vertex_buffer.clone()],
                set.clone(),
                (),
            )
            .unwrap()
    }

    /// Update texture with glyphs which are queued.
    fn flush_cache(&self) {
        // update texture cache
        self.cache
            .borrow_mut()
            .cache_queued(|rect, src_data| {
                self.update_required.replace(true);
                // println!("cached!");
                // Copy rectangle into the
                let width = (rect.max.x - rect.min.x) as usize;
                let height = (rect.max.y - rect.min.y) as usize;
                let mut dst_index = rect.min.y as usize * CACHE_WIDTH + rect.min.x as usize;
                let mut src_index = 0;

                for _ in 0..height {
                    let dst_slice =
                        &mut self.cache_pixel_buffer.borrow_mut()[dst_index..dst_index + width];
                    let src_slice = &src_data[src_index..src_index + width];
                    dst_slice.copy_from_slice(src_slice);

                    dst_index += CACHE_WIDTH;
                    src_index += width;
                }
            })
            .unwrap();
    }

    fn screen_to_gl(value: i32) -> f32 {
        let scaling = 0.001;
        // println!()
        let value2 = value as f32 * scaling;
        // println!("Value {}", value2);
        value2
    }

    /// Generate vertices for a positioned glyph
    fn get_glyph_vertices(&self, glyph: &PositionedGlyph) -> Vec<Vertex> {
        if let Ok(Some((uv_rect, screen_rect))) = self.cache.borrow().rect_for(0, glyph) {
            let gl_rect = Rect {
                min: point(
                    Self::screen_to_gl(screen_rect.min.x),
                    Self::screen_to_gl(screen_rect.min.y),
                ),
                max: point(
                    Self::screen_to_gl(screen_rect.max.x),
                    Self::screen_to_gl(screen_rect.max.y),
                ),
            };
            Self::quad_vertices(gl_rect, uv_rect)
        } else {
            vec![]
        }
    }

    /// Generate a single quad with proper texture coordinates
    fn quad_vertices(gl_rect: Rect<f32>, uv_rect: Rect<f32>) -> Vec<Vertex> {
        vec![
            // Lower left triangle, counter clockwise
            Vertex {
                position: [gl_rect.min.x, gl_rect.max.y],
                tex_position: [uv_rect.min.x, uv_rect.max.y],
            },
            Vertex {
                position: [gl_rect.min.x, gl_rect.min.y],
                tex_position: [uv_rect.min.x, uv_rect.min.y],
            },
            Vertex {
                position: [gl_rect.max.x, gl_rect.min.y],
                tex_position: [uv_rect.max.x, uv_rect.min.y],
            },
            // Top right triangle, counter clockwise
            Vertex {
                position: [gl_rect.max.x, gl_rect.min.y],
                tex_position: [uv_rect.max.x, uv_rect.min.y],
            },
            Vertex {
                position: [gl_rect.max.x, gl_rect.max.y],
                tex_position: [uv_rect.max.x, uv_rect.max.y],
            },
            Vertex {
                position: [gl_rect.min.x, gl_rect.max.y],
                tex_position: [uv_rect.min.x, uv_rect.max.y],
            },
        ]
    }
}

// Vertex type:

#[derive(Default, Debug, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_position: [f32; 2],
    // color: [f32; 4],
}
vulkano::impl_vertex!(Vertex, position, tex_position);

// Text shaders

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "

#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 tex_position;
layout(location = 0) out vec2 v_tex_position;

void main() {
    gl_Position = vec4(position, 0.2, 1.0);
    v_tex_position = tex_position;
}
"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "

#version 450

layout(location = 0) in vec2 v_tex_position;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform sampler2D tex;

void main() {
    float intensity = texture(tex, v_tex_position)[0];
    vec4 v_color = vec4(1.0, 1.0, 0.0, 1.0);
    f_color = v_color * intensity;
}

"
    }
}
