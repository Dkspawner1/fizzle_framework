use gl::types::*;
use std::ffi::CString;

pub struct Renderer {
    program: GLuint,
}

impl Renderer {
    pub fn new<F>(loader: F) -> Result<Self, String>
    where
        F: FnMut(&'static str) -> *const std::os::raw::c_void,
    {
        unsafe {
            gl::load_with(loader);

            let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER)?;
            let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER)?;
            let program = link_program(vs, fs)?;

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            Ok(Renderer { program })
        }
    }

    pub fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::UseProgram(self.program);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

fn compile_shader(src: &str, ty: GLenum) -> Result<GLuint, String> {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    let mut status = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    }

    if status != (gl::TRUE as GLint) {
        let mut len = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }
        let mut buf = Vec::with_capacity(len as usize);
        unsafe {
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }
        return Err(String::from_utf8(buf).unwrap());
    }

    Ok(shader)
}

fn link_program(vs: GLuint, fs: GLuint) -> Result<GLuint, String> {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            return Err(String::from_utf8(buf).unwrap());
        }
        Ok(program)
    }
}

const VS_SRC: &'static str = "
#version 330 core
layout (location = 0) in vec3 aPos;
void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

const FS_SRC: &'static str = "
#version 330 core
out vec4 FragColor;
void main() {
    FragColor = vec4(1.0, 0.5, 0.2, 1.0);
}";