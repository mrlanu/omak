use crate::renderer::shader::Shader;
use std::collections::HashMap;
use std::fs;

pub struct ResourcesManager {
    cached_shaders: HashMap<String, Shader>,
}

impl ResourcesManager {
    pub fn new() -> Self {
        Self {
            cached_shaders: HashMap::new(),
        }
    }
    pub fn load_shader(&mut self, shader_path: &str) -> &mut Shader {
        let name = format!("{shader_path}");
        if self.cached_shaders.contains_key(&name) {
            return self.cached_shaders.get_mut(&name).unwrap();
        }
        let shader_source = self.parse_shader(shader_path);
        let new_shader = Shader::new(shader_source);
        self.cached_shaders.insert(name.clone(), new_shader);
        self.cached_shaders.get_mut(&name).unwrap()
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
