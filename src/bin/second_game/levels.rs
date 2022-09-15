use crate::{TILES_IN_HEIGHT, TILES_IN_WIDTH, TILE_SIZE_SCALED};
use nalgebra_glm as glm;
use omak::renderer::texture::{self, SpritesBuilder, Texture};
use omak::renderer::{ImgKind, Renderer};
use std::path::Path;
pub struct Level {
    level_data: Vec<u8>,
}
impl Level {
    pub fn new(level_image: &str) -> Self {
        let mut level_data = Vec::new();
        let image = image::open(&Path::new(level_image)).expect("Failed to load an image");
        for (x, y, pixel) in image.to_rgb8().enumerate_pixels_mut() {
            let image::Rgb(data) = *pixel;
            level_data.insert(
                texture::get_index(x as usize, y as usize, image.width() as usize),
                data[0],
            );
        }
        Self { level_data }
    }

    pub fn get_sprite_index(&self, x: usize, y: usize) -> usize {
        self.level_data[texture::get_index(x, y, TILES_IN_WIDTH as usize) as usize] as usize
    }
}

pub struct LevelManager {
    sprites: Vec<Texture>,
    level: Level,
}
impl LevelManager {
    pub fn new() -> Self {
        Self {
            sprites: SpritesBuilder::init("resources/img/outside_sprites.png", ImgKind::PNG)
                .with_rows(4, 32)
                .with_columns(12, 32)
                .build(),
            level: Level::new("resources/img/level_one_data.png"),
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        for y in 0..TILES_IN_HEIGHT as i32 {
            for x in 0..TILES_IN_WIDTH as i32 {
                renderer.draw_image(
                    glm::vec2(x as f32 * TILE_SIZE_SCALED, y as f32 * TILE_SIZE_SCALED),
                    glm::vec2(TILE_SIZE_SCALED, TILE_SIZE_SCALED),
                    0.0,
                    glm::vec3(1.0, 1.0, 1.0),
                    &self.sprites[self.level.get_sprite_index(x as usize, y as usize)],
                );
            }
        }
    }
}
