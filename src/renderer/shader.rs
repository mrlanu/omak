use gl::types::*;
use std::ffi::CString;
use std::ptr;

#[derive(Clone)]
pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(source: (String, String)) -> Self {
        let (vertex_code, fragment_code) = source;
        let vertex_src = CString::new(vertex_code.as_bytes()).unwrap();
        let fragment_src = CString::new(fragment_code.as_bytes()).unwrap();

        let id;
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex_shader, 1, &vertex_src.as_ptr(), ptr::null());
            gl::CompileShader(vertex_shader);
            compile_errors(vertex_shader, "VERTEX");
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment_shader, 1, &fragment_src.as_ptr(), ptr::null());
            gl::CompileShader(fragment_shader);
            compile_errors(fragment_shader, "FRAGMENT");
            id = gl::CreateProgram();
            gl::AttachShader(id, vertex_shader);
            gl::AttachShader(id, fragment_shader);
            gl::LinkProgram(id);
            compile_errors(id, "PROGRAM");
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
        Self { id }
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn compile_errors(shader_id: u32, tp: &str) {
    // Stores status of compilation
    let mut has_compiled = gl::FALSE as GLint;
    // Character array to store error message in
    let mut info_log: Vec<u8> = Vec::with_capacity(1024);
    if tp != "PROGRAM" {
        unsafe {
            gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut has_compiled);
        }
        if has_compiled == gl::FALSE as GLint {
            unsafe {
                gl::GetShaderInfoLog(
                    shader_id,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
            }
            println!(
                "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                tp,
                std::str::from_utf8(&info_log[..]).unwrap()
            );
        }
    } else {
        unsafe {
            gl::GetProgramiv(shader_id, gl::LINK_STATUS, &mut has_compiled);
        }
        if has_compiled == gl::FALSE as GLint {
            unsafe {
                gl::GetProgramInfoLog(
                    shader_id,
                    1024,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
            }
            println!(
                "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                tp,
                std::str::from_utf8(&info_log[..]).unwrap()
            );
        }
    }
}
