use crate::renderer::ImgKind;
use gl::types::*;
use image::DynamicImage;
use std::os::raw::c_void;

#[derive(Clone)]
pub struct Texture {
    pub id: u32,
}
impl Texture {
    pub fn new(image: DynamicImage, kind: ImgKind) -> Self {
        println!("New Texture");
        let img_bytes = image.as_bytes();

        let mut id = 0;
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::GenTextures(1, &mut id);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                match kind {
                    ImgKind::PNG => gl::RGBA,
                    ImgKind::JPEG => gl::RGB,
                    ImgKind::JPG => gl::RGB,
                } as i32,
                image.width() as i32,
                image.height() as i32,
                0,
                match kind {
                    ImgKind::PNG => gl::RGBA,
                    ImgKind::JPEG => gl::RGB,
                    ImgKind::JPG => gl::RGB,
                } as u32,
                gl::UNSIGNED_BYTE,
                &img_bytes[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        Self { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn delete(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}
