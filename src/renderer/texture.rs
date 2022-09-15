use crate::renderer::ImgKind;
use gl::types::*;
use image::DynamicImage;
use nalgebra_glm as glm;
use std::os::raw::c_void;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Texture {
    pub id: u32,
    pub width: u32,
    pub height: u32,
}
impl Texture {
    fn new(image: DynamicImage, kind: ImgKind) -> Self {
        let width = image.width();
        let height = image.height();

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
        Self { id, width, height }
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

pub struct TextureBuilder {
    subimg: Option<glm::UVec4>,
    img_path: String,
    image_kind: ImgKind,
}
impl TextureBuilder {
    pub fn init(img_path: &str, image_kind: ImgKind) -> Self {
        Self {
            img_path: img_path.to_string(),
            image_kind,
            subimg: None,
        }
    }
    pub fn subimg(mut self, subimg: glm::UVec4) -> Self {
        self.subimg = Some(subimg);
        self
    }
    pub fn build(self) -> Texture {
        let mut image = load_image_from_file(&self.img_path);
        if let Some(subimage) = self.subimg {
            let subimg =
                image::imageops::crop(&mut image, subimage.x, subimage.y, subimage.z, subimage.w)
                    .to_image();
            return Texture::new(image::DynamicImage::ImageRgba8(subimg), self.image_kind);
        }
        Texture::new(image, self.image_kind)
    }
}

pub struct SpritesBuilder {
    img_path: String,
    image_kind: ImgKind,
    rows: usize,
    columns: usize,
    sprite_width: u32,
    sprite_height: u32,
}

impl SpritesBuilder {
    pub fn init(img_path: &str, image_kind: ImgKind) -> Self {
        Self {
            img_path: img_path.to_string(),
            image_kind,
            rows: 0,
            columns: 0,
            sprite_width: 0,
            sprite_height: 0,
        }
    }
    /// amount: How many rows on the atlas image
    ///
    ///sprite_width: width in pixels
    pub fn with_rows(mut self, amount: usize, sprite_width: u32) -> Self {
        self.rows = amount;
        self.sprite_width = sprite_width;
        self
    }

    pub fn with_columns(mut self, amount: usize, sprite_height: u32) -> Self {
        self.columns = amount;
        self.sprite_height = sprite_height;
        self
    }
    pub fn build(self) -> Vec<Texture> {
        let mut image = load_image_from_file(&self.img_path);
        let mut textures = Vec::new();
        for row in 0..self.rows {
            for column in 0..self.columns {
                let subimg = image::imageops::crop(
                    &mut image,
                    column as u32 * self.sprite_width,
                    row as u32 * self.sprite_height,
                    self.sprite_width,
                    self.sprite_height,
                )
                .to_image();
                let tex = Texture::new(
                    image::DynamicImage::ImageRgba8(subimg),
                    self.image_kind.clone(),
                );

                textures.insert(get_index(column, row, self.columns), tex);
            }
        }
        println!("Loaded {} textures.", textures.len());
        textures
    }
}

fn load_image_from_file(img_path: &str) -> DynamicImage {
    image::open(&Path::new(img_path)).expect("Failed to load an image")
}

pub fn get_index(x: usize, y: usize, max_x: usize) -> usize {
    (y * max_x) + x
}
