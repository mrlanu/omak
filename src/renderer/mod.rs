mod gl_objects;
mod shader;
pub mod texture;
pub mod utils;

use self::utils::ResourcesManager;
use gl::types::*;
use gl_objects::{MyTypes, VertexBufferElement, VertexesLayout, EBO, VAO, VBO};
use shader::Shader;
use std::collections::HashMap;
use std::ffi::CString;
use std::mem;
use std::ptr;
use texture::Texture;

pub enum ImgKind {
    PNG,
    JPEG,
    JPG,
}

pub struct Renderer {
    pub width: f32,
    pub height: f32,
    pub res_manager: ResourcesManager,
}
impl Renderer {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            res_manager: ResourcesManager::new(),
        }
    }
    pub fn draw(&mut self, vao: &VAO, vbo: &VBO) {
        // self.shader.activate();
        self.res_manager.load_shader("sprite.shader").activate();
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

    pub fn draw_object(&mut self, obj: &RenderObject) {
        self.res_manager.load_shader("basic.shader").activate();
        obj.vao.bind();
        obj.texture.as_ref().unwrap().bind();
        unsafe {
            if obj.ebo.is_some() {
                gl::DrawElements(
                    gl::TRIANGLES,
                    obj.vbo.count as GLsizei,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, obj.vbo.count as GLsizei);
            }
        }
        obj.texture.as_ref().unwrap().unbind();
    }

    pub fn draw_image(
        &mut self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        img_name: &str,
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
            .texture(img_name, img_kind)
            .build();
        self.draw_object(&img);
    }

    pub fn draw_sprite(
        &mut self,
        position: nalgebra_glm::Vec2,
        size: nalgebra_glm::Vec2,
        rotate: f32,
        color: nalgebra_glm::Vec3,
    ) {
        let sprite = RenderObjectBuilder::new()
            .vertices(vec![
                // pos      // tex
                0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
                1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
            ])
            .layout(MyTypes::FLOAT, 4)
            .texture("my_s.png", ImgKind::PNG)
            .build();

        let mut shader = self.res_manager.load_shader("sprite.shader");
        shader.activate();

        let mut model: nalgebra_glm::Mat4 = nalgebra_glm::mat4(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        model = nalgebra_glm::translate(&model, &nalgebra_glm::vec3(position.x, position.y, 0.0));
        // model = nalgebra_glm::rotate(&model, *nalgebra_glm::radians(rotate), &nalgebra_glm::vec3(0.0, 0.0, 1.0)); // then rotatemodel
        model = nalgebra_glm::translate(
            &model,
            &nalgebra_glm::vec3(-0.5 * size.x, -0.5 * size.y, 0.0),
        ); // move origin back

        model = nalgebra_glm::scale(&model, &nalgebra_glm::vec3(size.x, size.y, 1.0));
        shader.set_matrix4("model", &model);

        // render textured quad
        shader.set_vector_3f("spriteColor", color.x, color.y, color.z);
        // unsafe {
        //     gl::ActiveTexture(gl::TEXTURE0);
        //     texture.bind();
        //
        //     gl::BindVertexArray(sprite.vao.id);
        //     gl::DrawArrays(gl::TRIANGLES, 0, 6);
        //     gl::BindVertexArray(0);
        // }
        self.draw_object(&sprite);
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

pub struct RenderObject {
    _vertices: Vec<f32>,
    _indices: Vec<i32>,
    vao: VAO,
    vbo: VBO,
    ebo: Option<EBO>,
    texture: Option<Texture>,
}
impl RenderObject {
    pub fn destroy(&mut self) {
        self.vao.delete();
        self.vbo.delete();
        if let Some(_) = self.ebo {
            self.ebo.as_mut().unwrap().delete();
        }
        if let Some(_) = self.texture {
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

    fn texture(mut self, name: &str, img_kind: ImgKind) -> Self {
        let mut res = ResourcesManager::new();
        self.texture = res.load_texture(name, img_kind);
        self.texture.as_ref().unwrap().unbind();
        self
    }

    fn build(self) -> RenderObject {
        self.vao.link(&self.vbo.as_ref().unwrap(), &self.layouts);
        self.vao.unbind();
        self.vbo.as_ref().unwrap().unbind();
        if self.ebo.is_some() {
            self.ebo.as_ref().unwrap().unbind();
        }

        RenderObject {
            _vertices: self.vertices,
            _indices: self.indices,
            vao: self.vao,
            vbo: self.vbo.unwrap(),
            ebo: if self.ebo.is_some() { self.ebo } else { None },
            texture: if self.texture.is_some() {
                self.texture
            } else {
                None
            },
        }
    }
}
