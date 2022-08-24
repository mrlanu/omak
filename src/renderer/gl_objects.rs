use gl::types::*;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub struct VBO {
    pub id: u32,
    pub count: usize,
}
impl VBO {
    pub fn new(vertices: &[f32]) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &vertices[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );
        }
        Self {
            id,
            count: vertices.len(),
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn delete(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}

//-------------------------------------

pub struct EBO {
    pub id: u32,
}
impl EBO {
    pub fn new(indices: &[i32]) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &indices[0] as *const i32 as *const c_void,
                gl::STATIC_DRAW,
            );
        }
        Self { id }
    }

    pub fn _bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn delete(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}

//-------------------------------------

pub struct VAO {
    pub id: u32,
}
impl VAO {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Self { id }
    }

    pub fn link(&self, vbo: &VBO, layout: &VertexesLayout) {
        vbo.bind();
        let mut offset = 0;
        for (i, el) in layout.elements.iter().enumerate() {
            let (tp, size) = match el.my_type {
                MyTypes::FLOAT => (gl::FLOAT, mem::size_of::<GLfloat>()),
                MyTypes::INT => (gl::UNSIGNED_INT, mem::size_of::<GLsizei>()),
                MyTypes::CHAR => (gl::BYTE, mem::size_of::<GLchar>()),
            };
            unsafe {
                gl::VertexAttribPointer(
                    i as u32,
                    el.size as GLsizei,
                    tp,
                    el.normalized as u8,
                    layout.stride as GLsizei,
                    if offset == 0 {
                        ptr::null()
                    } else {
                        (offset * size) as *const c_void
                    },
                );
                gl::EnableVertexAttribArray(i as u32);
                offset += el.size;
            }
        }
        vbo.unbind();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn delete(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &mut self.id) }
    }
}

pub struct VertexesLayout {
    pub elements: Vec<VertexBufferElement>,
    pub stride: i32,
}
impl VertexesLayout {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            stride: 0,
        }
    }

    pub fn push_el(&mut self, tp: MyTypes, size: usize) {
        let el = VertexBufferElement::new(size, tp, false);
        self.elements.push(el);
        self.stride += match tp {
            MyTypes::FLOAT => mem::size_of::<GLfloat>(),
            MyTypes::INT => mem::size_of::<GLsizei>(),
            MyTypes::CHAR => mem::size_of::<GLchar>(),
        } as GLsizei
            * size as GLsizei;
    }
}

pub struct VertexBufferElement {
    size: usize,
    my_type: MyTypes,
    normalized: bool,
}
impl VertexBufferElement {
    pub fn new(size: usize, tp: MyTypes, normalized: bool) -> Self {
        Self {
            size,
            my_type: tp,
            normalized,
        }
    }
}

#[derive(Clone, Copy)]
pub enum MyTypes {
    FLOAT,
    INT,
    CHAR,
}
