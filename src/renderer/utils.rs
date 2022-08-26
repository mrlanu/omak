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
    pub fn load_shader(&mut self, name: &str) -> &mut Shader {
        if self.cached_shaders.contains_key(name) {
            return self.cached_shaders.get_mut(name).unwrap();
        }
        let shader_source = self.parse_shader(format!("{}/{}", SHADERS_PATH, name).as_str());
        let new_shader = Shader::new(shader_source);
        self.cached_shaders.insert(name.to_string(), new_shader);
        self.cached_shaders.get_mut(name).unwrap()
    }

    pub fn load_texture(&mut self, name: &str, image_kind: ImgKind) -> &mut Texture {
        if self.cached_textures.contains_key(name) {
            return self.cached_textures.get_mut(name).unwrap();
        }
        let image = self.load_image_from_file(name);
        let texture = Texture::new(image, image_kind);
        self.cached_textures
            .insert(name.to_string(), texture.clone());
        self.cached_textures.get_mut(name).unwrap()
    }

    fn load_image_from_file(&self, name: &str) -> DynamicImage {
        let img_orig = image::open(&Path::new(format!("{}/{}", IMAGES_PATH, name).as_str()))
            .expect("Failed to load an image");
        img_orig.flipv()
    }

    fn parse_shader(&mut self, path: &str) -> (String, String) {
        let mut kind = -1;
        let (mut vertex, mut fragment) = (String::new(), String::new());
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
