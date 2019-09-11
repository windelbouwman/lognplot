/// Demo with SDL2 as a system to draw stuff!
#[macro_use]
extern crate log;

use imgui::render_gl::{Program, Shader};

fn main() {
    simple_logger::init().unwrap();

    info!("Booting 1337-h4xx0r-code version 1.0");

    let _sdl_context = sdl2::init().unwrap();

    let video_subsystem = _sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("F)))))", 1000, 300)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    let gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Use gl shaders
    let vertex_shader = Shader::from_vert_source(include_str!("triangle.vert")).unwrap();
    let fragment_shader = Shader::from_frag_source(include_str!("triangle.frag")).unwrap();
    let prog = Program::from_shaders(&[vertex_shader, fragment_shader]).unwrap();

    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        
        // continue here
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null() // offset of the first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }


    let mut event_pump = _sdl_context.event_pump().unwrap();
    info!("Entering event loopz0r!");
    'main: loop {
        // debug!("Loop iteration!");
        for event in event_pump.poll_iter() {
            debug!("{:?}", event);
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'main;
                }
                sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Q),
                    ..
                } => {
                    break 'main;
                }
                _ => {}
            }
        }

        unsafe {
            gl::Viewport(0, 0, 900, 700); // set viewport
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        prog.set_use();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }


        window.gl_swap_window();
    }

    info!("We are done with tha main loopz0rr!!!");
}
