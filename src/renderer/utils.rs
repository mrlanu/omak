use image::DynamicImage;

use crate::renderer::shader::Shader;
use crate::renderer::texture::Texture;
use crate::renderer::ImgKind;
use nalgebra_glm as glm;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

pub struct ResourcesManager {
    cached_shaders: HashMap<u64, Shader>,
    cached_textures: HashMap<u64, Texture>,
}

impl ResourcesManager {
    pub fn new() -> Self {
        Self {
            cached_textures: HashMap::new(),
            cached_shaders: HashMap::new(),
        }
    }
    pub fn load_shader(&mut self, shader_path: &str) -> &mut Shader {
        let key = get_hash(format!("{shader_path}"));
        if self.cached_shaders.contains_key(&key) {
            return self.cached_shaders.get_mut(&key).unwrap();
        }
        let shader_source = self.parse_shader(shader_path);
        let new_shader = Shader::new(shader_source);
        self.cached_shaders.insert(key, new_shader);
        self.cached_shaders.get_mut(&key).unwrap()
    }

    pub fn load_texture(&mut self, img_path: &str, image_kind: ImgKind) -> &mut Texture {
        let key = get_hash(format!("{img_path}"));
        if self.cached_textures.contains_key(&key) {
            return self.cached_textures.get_mut(&key).unwrap();
        }
        let image = self.load_image_from_file(img_path);
        self.cached_textures
            .insert(key, Texture::new(image, image_kind));
        self.cached_textures.get_mut(&key).unwrap()
    }

    pub fn load_texture_partial(
        &mut self,
        subimg: glm::UVec4,
        img_path: &str,
        image_kind: ImgKind,
    ) -> &mut Texture {
        let key = get_hash(format!(
            "{img_path}-{}-{}-{}-{}",
            subimg.x, subimg.y, subimg.z, subimg.w
        ));
        if self.cached_textures.contains_key(&key) {
            return self.cached_textures.get_mut(&key).unwrap();
        }
        let mut image = self.load_image_from_file(img_path);
        let subimg =
            image::imageops::crop(&mut image, subimg.x, subimg.y, subimg.z, subimg.w).to_image();

        self.cached_textures.insert(
            key,
            Texture::new(image::DynamicImage::ImageRgba8(subimg), image_kind),
        );
        self.cached_textures.get_mut(&key).unwrap()
    }

    fn load_image_from_file(&self, img_path: &str) -> DynamicImage {
        image::open(&Path::new(img_path)).expect("Failed to load an image")
    }

    fn parse_shader(&mut self, shader_path: &str) -> (String, String) {
        let mut kind = -1;
        let (mut vertex, mut fragment) = (String::new(), String::new());
        let contents =
            fs::read_to_string(&shader_path).expect("Should have been able to read the file");
        for line in contents.lines() {
            if line.contains("#shader") {
                if line.contains("vertex") {
                    kind = 0;
                } else if line.contains("fragment") {
                    kind = 1;
                }
            } else {
                match kind {
                    0 => {
                        vertex.push_str(&line);
                        vertex.push_str("\n");
                    }
                    1 => {
                        fragment.push_str(&line);
                        fragment.push_str("\n");
                    }
                    _ => {}
                }
            }
        }
        (vertex, fragment)
    }
}

fn get_hash(input: String) -> u64 {
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}
