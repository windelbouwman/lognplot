use super::shader::Shader;
use super::utils::create_white_space_cstring_with_len;

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    fn new() -> Result<Self, String> {
        let program_id = unsafe { gl::CreateProgram() };

        Ok(Self { id: program_id })
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let program = Self::new()?;

        for shader in shaders {
            program.attach_shader(shader);
        }
        program.link()?;

        for shader in shaders {
            program.detach_shader(shader);
        }

        Ok(program)
    }

    fn attach_shader(&self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.id, shader.id());
        }
    }

    fn detach_shader(&self, shader: &Shader) {
        unsafe {
            gl::DetachShader(self.id, shader.id());
        }
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
