mod gl_objects;
mod shader;
pub mod texture;
pub mod utils;

use self::utils::ResourcesManager;
use gl::types::*;
use gl_objects::{MyTypes, VertexBufferElement, VertexesLayout, EBO, VAO, VBO};
use nalgebra_glm as glm;
use std::mem;
use std::ptr;
use texture::Texture;

pub enum ImgKind {
    PNG,
    JPEG,
    JPG,
}

pub struct Renderer {
    gl_objects: GlObjects,
    pub res_manager: ResourcesManager,
}
impl Renderer {
    pub fn new() -> Self {
        let gl_objects = GlObjectsBuilder::new()
            .vertices(vec![
                // pos      // tex
                0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
            ])
            .layout(MyTypes::FLOAT, 4)
            .build();

        Self {
            gl_objects,
            res_manager: ResourcesManager::new(),
        }
    }

    pub fn draw_image(
        &mut self,
        position: glm::Vec2,
        size: glm::Vec2,
        rotate: f32,
        color: glm::Vec3,
        image: &str,
    ) {
        let mut model = glm::Mat4x4::from_diagonal_element(1.0);
        model = glm::translate(&model, &glm::vec3(position.x, position.y, 0.0));
        model = glm::rotate(&model, rotate, &glm::vec3(0.0, 0.0, 1.0));
        model = glm::translate(&model, &glm::vec3(-0.5 * size.x, -0.5 * size.y, 0.0)); // move
        model = glm::scale(&model, &glm::vec3(size.x, size.y, 1.0));

        let shader = self.res_manager.load_shader("sprite.shader");
        shader.activate();
        shader.set_matrix4("model", &model);
        shader.set_vector_3f("spriteColor", color.x, color.y, color.z);

        self.gl_objects.vao.bind();

        let texture = self.res_manager.load_texture(image, ImgKind::PNG);
        texture.bind();

        unsafe {
            if self.gl_objects.ebo.is_some() {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.gl_objects.vbo.count as GLsizei,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, self.gl_objects.vbo.count as GLsizei);
            }
        }
        self.gl_objects.vao.unbind();
        texture.unbind();
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
}

pub struct GlObjects {
    _vertices: Vec<f32>,
    _indices: Vec<i32>,
    vao: VAO,
    vbo: VBO,
    ebo: Option<EBO>,
}
impl GlObjects {
    pub fn destroy(&mut self) {
        self.vao.delete();
        self.vbo.delete();
        if let Some(_) = self.ebo {
            self.ebo.as_mut().unwrap().delete();
        }
    }
}

struct GlObjectsBuilder {
    vertices: Vec<f32>,
    indices: Vec<i32>,
    vao: VAO,
    vbo: Option<VBO>,
    ebo: Option<EBO>,
    layouts: VertexesLayout,
    texture: Option<Texture>,
}
impl GlObjectsBuilder {
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

    fn build(self) -> GlObjects {
        self.vao.link(&self.vbo.as_ref().unwrap(), &self.layouts);
        self.vao.unbind();
        self.vbo.as_ref().unwrap().unbind();
        if self.ebo.is_some() {
            self.ebo.as_ref().unwrap().unbind();
        }

        GlObjects {
            _vertices: self.vertices,
            _indices: self.indices,
            vao: self.vao,
            vbo: self.vbo.unwrap(),
            ebo: if self.ebo.is_some() { self.ebo } else { None },
        }
    }
}
