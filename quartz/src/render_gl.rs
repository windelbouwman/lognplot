/// Spiffy rust wrapper around programs and shaders!
///
///

// use clib
use std::ffi::{CStr, CString};

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    fn new() -> Result<Self, String> {
        let program_id = unsafe {
            gl::CreateProgram()
        };

        Ok(Self {
            id: program_id
        })
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let program = Self::new()?;

        for shader in shaders {
            unsafe {
                gl::AttachShader(program.id, shader.id());
            }
        }
        program.link()?;

        for shader in shaders {
            unsafe {
                gl::DetachShader(program.id, shader.id());
            }
        }

        Ok(program)
    }

    fn link(&self) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(self.id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let msg = download_program_gl_error(self.id);
            Err(msg)
        } else {
            Ok(())
        }
    }

    /// Put this program to work!
    pub fn set_use(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }

}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn from_vert_source(source: &str) -> Result<Self, String> {
        Self::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &str) -> Result<Self, String> {
        Self::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn from_source(source: &str, shader_kind: gl::types::GLuint) -> Result<Self, String> {
        let source = CString::new(source).unwrap();
        compile_shader(&source, shader_kind)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn compile_shader(src: &CStr, shader_kind: gl::types::GLuint) -> Result<Shader, String> {
    let id: gl::types::GLuint = unsafe { gl::CreateShader(shader_kind) };

    unsafe {
        gl::ShaderSource(id, 1, &src.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0 {
        Err(download_shader_gl_error(id))
    } else {
        Ok(Shader { id })
    }

    // fn get_log() {
    // gl::GetShaderiv(id);
    // }
}

fn download_shader_gl_error(id: gl::types::GLuint) -> String {
    let mut len: gl::types::GLint = 0;
    unsafe {
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
    }

    let error = create_white_space_cstring_with_len(len as usize);

    unsafe {
        gl::GetShaderInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar,
        );
    }

    let msg = error.to_string_lossy().into_owned();
    msg
}

/// Download program error message from gl
fn download_program_gl_error(id: gl::types::GLuint) -> String {
    let mut len: gl::types::GLint = 0;
    unsafe {
        gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
    }

    let error = create_white_space_cstring_with_len(len as usize);

    unsafe {
        gl::GetProgramInfoLog(
            id,
            len,
            std::ptr::null_mut(),
            error.as_ptr() as *mut gl::types::GLchar,
        );
    }

    let msg = error.to_string_lossy().into_owned();
    msg
}

fn create_white_space_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
