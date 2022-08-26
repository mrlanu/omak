use image::DynamicImage;

use crate::renderer::shader::Shader;
use crate::renderer::texture::Texture;
use crate::renderer::ImgKind;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const IMAGES_PATH: &str = "resources/img";
const SHADERS_PATH: &str = "resources/shaders";

pub struct ResourcesManager {
    cached_shaders: HashMap<String, Shader>,
    cached_textures: HashMap<String, Texture>,
}

impl ResourcesManager {
    pub fn new() -> Self {
        Self {
            cached_textures: HashMap::new(),
            cached_shaders: HashMap::new(),
        }
    }
    pub fn load_shader(&mut self, name: &str) -> Shader {
        if let Some(shader) = self.cached_shaders.get(name) {
            return shader.clone();
        }
        let shader_source = self.parse_shader(format!("{}/{}", SHADERS_PATH, name).as_str());
        let new_shader = Shader::new(shader_source);
        self.cached_shaders
            .insert(name.to_string(), new_shader.clone());
        new_shader
    }

    pub fn load_texture(&mut self, name: &str, image_kind: ImgKind) -> Option<Texture> {
        if let Some(texture) = self.cached_textures.get(name) {
            return Some(texture.clone());
        }
        let image = self.load_image_from_file(name);
        let texture = Texture::new(image, image_kind);
        self.cached_textures
            .insert(name.to_string(), texture.clone());
        Some(texture)
    }

    fn load_image_from_file(&self, name: &str) -> DynamicImage {
        let img_orig = image::open(&Path::new(format!("{}/{}", IMAGES_PATH, name).as_str()))
            .expect("Failed to load an image");
        img_orig.flipv()
    }

    fn parse_shader(&self, path: &str) -> (String, String) {
        let mut kind = -1;
        let mut vertex = String::new();
        let mut fragment = String::new();
        let contents = fs::read_to_string(&path).expect("Should have been able to read the file");
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
