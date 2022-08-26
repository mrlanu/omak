use gl::types::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;

#[derive(Clone)]
pub struct Shader {
    pub id: u32,
    cache_uniform_location: HashMap<String, i32>,
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
        Self {
            id,
            cache_uniform_location: HashMap::new(),
        }
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

    pub fn set_uniform_4f(&mut self, name: &str, v0: f32, v1: f32, v2: f32, v3: f32) {
        unsafe {
            gl::Uniform4f(self.get_uniform_location(name), v0, v1, v2, v3);
        }
    }

    pub fn set_uniform_1i(&mut self, name: &str, v: i32) {
        unsafe {
            gl::Uniform1i(self.get_uniform_location(name), v);
        }
    }

    pub fn set_uniform_1f(&mut self, name: &str, v: f32) {
        unsafe {
            gl::Uniform1f(self.get_uniform_location(name), v);
        }
    }

    pub fn set_vector_3f(&mut self, name: &str, v0: f32, v1: f32, v2: f32) {
        unsafe {
            gl::Uniform3f(self.get_uniform_location(name), v0, v1, v2);
        }
    }

    pub fn set_matrix4(&mut self, name: &str, matrix: &nalgebra_glm::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(self.get_uniform_location(name), 1, 0, matrix.as_ptr());
        }
    }

    fn get_uniform_location(&mut self, name: &str) -> i32 {
        if self.cache_uniform_location.get(name).is_some() {
            return self.cache_uniform_location.get(name).unwrap().clone();
        }
        let var_name = CString::new(name.as_bytes()).unwrap();
        let location;
        unsafe {
            location = gl::GetUniformLocation(self.id, var_name.as_ptr());
            if location == -1 {
                println!("Uniform {} doesnt exist!", name);
            }
            self.cache_uniform_location
                .insert(name.to_string(), location);
            println!("Map {:?}", self.cache_uniform_location);
            location
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
