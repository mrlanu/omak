mod gl_objects;
mod shader;
mod texture;

use gl::types::*;
use gl_objects::{MyTypes, VertexBufferElement, VertexesLayout, EBO, VAO, VBO};
use shader::Shader;
use std::collections::HashMap;
use std::ffi::CString;
use std::mem;
use std::ptr;
use texture::{ImgKind, Texture};

pub struct Renderer {
    pub width: f32,
    pub height: f32,
    pub shader: Shader,
    cache_uniform_location: HashMap<String, i32>,
}
impl Renderer {
    pub fn new(width: f32, height: f32, shader_src_path: &str) -> Self {
        Self {
            width,
            height,
            shader: Shader::new(shader_src_path),
            cache_uniform_location: HashMap::new(),
        }
    }
    pub fn draw(&self, vao: &VAO, vbo: &VBO) {
        self.shader.activate();
        vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                vbo.count as GLsizei,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }

    pub fn draw_object(&self, obj: &RenderObject) {
        self.shader.activate();
        obj.vao.bind();
        obj.texture.as_ref().unwrap().bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                obj.vbo.count as GLsizei,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
        obj.texture.as_ref().unwrap().unbind();
    }

    pub fn draw_image(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        img_path: &str,
        img_kind: ImgKind,
    ) {
        let mut vertices = Vec::new();
        let gl_x1 = ((x + width) - self.width / 2.0) * (2.0 / self.width);
        let gl_y1 = ((y + height) - self.height / 2.0) * (2.0 / self.height);
        let gl_x2 = (x - self.width / 2.0) * (2.0 / self.width);
        let gl_y2 = (y - self.height / 2.0) * (2.0 / self.height);
        vertices.append(&mut vec![
            gl_x2, gl_y1, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, gl_x2, gl_y2, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0,
            gl_x1, gl_y2, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, gl_x1, gl_y1, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0,
        ]);
        let img = RenderObjectBuilder::new()
            .vertices(vertices)
            .indices(vec![0, 1, 3, 1, 2, 3])
            .layout(MyTypes::FLOAT, 3)
            .layout(MyTypes::FLOAT, 3)
            .layout(MyTypes::FLOAT, 2)
            .texture(img_path, img_kind)
            .build();
        self.draw_object(&img);
    }

    pub fn clear(&self) {
        unsafe {
            // gl::ClearColor(0.07, 0.13, 0.17, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn print_gl_version(&self) {
        unsafe {
            let (mut major, mut minor) = (0, 0);
            gl::GetIntegerv(gl::MAJOR_VERSION, &mut major);
            gl::GetIntegerv(gl::MINOR_VERSION, &mut minor);
            println!("Version: {}.{}", major, minor);
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

    fn get_uniform_location(&mut self, name: &str) -> i32 {
        if self.cache_uniform_location.get(name).is_some() {
            return self.cache_uniform_location.get(name).unwrap().clone();
        }
        let var_name = CString::new(name.as_bytes()).unwrap();
        let location;
        unsafe {
            location = gl::GetUniformLocation(self.shader.id, var_name.as_ptr());
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

pub struct RenderObject {
    _vertices: Vec<f32>,
    _indices: Vec<i32>,
    vao: VAO,
    vbo: VBO,
    ebo: EBO,
    texture: Option<Texture>,
}
impl RenderObject {
    pub fn destroy(&mut self) {
        self.vao.delete();
        self.vbo.delete();
        self.ebo.delete();
        if self.texture.is_some() {
            self.texture.as_mut().unwrap().delete();
        }
    }
}

struct RenderObjectBuilder {
    vertices: Vec<f32>,
    indices: Vec<i32>,
    vao: VAO,
    vbo: Option<VBO>,
    ebo: Option<EBO>,
    layouts: VertexesLayout,
    texture: Option<Texture>,
}
impl RenderObjectBuilder {
    pub fn new() -> Self {
        let vao = VAO::new();
        vao.bind();
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            vao,
            vbo: None,
            ebo: None,
            layouts: VertexesLayout::new(),
            texture: None,
        }
    }

    fn vertices(mut self, vertices: Vec<f32>) -> Self {
        self.vertices = vertices;
        self.vbo = Some(VBO::new(&self.vertices[..]));
        self
    }

    fn indices(mut self, indices: Vec<i32>) -> Self {
        self.indices = indices;
        self.ebo = Some(EBO::new(&self.indices[..]));
        self
    }

    fn layout(mut self, tp: MyTypes, size: usize) -> Self {
        let el = VertexBufferElement::new(size, tp, false);
        self.layouts.elements.push(el);
        self.layouts.stride += match tp {
            MyTypes::FLOAT => mem::size_of::<GLfloat>(),
            MyTypes::INT => mem::size_of::<GLsizei>(),
            MyTypes::CHAR => mem::size_of::<GLchar>(),
        } as GLsizei
            * size as GLsizei;
        self
    }

    fn texture(mut self, path: &str, img_kind: ImgKind) -> Self {
        self.texture = Some(Texture::new(path, img_kind));
        self.texture.as_ref().unwrap().unbind();
        self
    }

    fn build(self) -> RenderObject {
        self.vao.link(&self.vbo.as_ref().unwrap(), &self.layouts);
        self.vao.unbind();
        self.vbo.as_ref().unwrap().unbind();
        self.ebo.as_ref().unwrap().unbind();

        RenderObject {
            _vertices: self.vertices,
            _indices: self.indices,
            vao: self.vao,
            vbo: self.vbo.unwrap(),
            ebo: self.ebo.unwrap(),
            texture: self.texture,
        }
    }
}
