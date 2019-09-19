use crate::canvas::Color;

pub struct Rectangle {}

impl Rectangle {
    pub fn new(color: Color) -> Self {
        Rectangle {}
    }
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
#version 450

layout(location = 0) in vec2 position;

void main() {
gl_Position = vec4(position, 0.1, 1.0);
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
f_color = vec4(1.0, 1.0, 0.0, 1.0);
}
"
    }
}
