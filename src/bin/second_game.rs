use nalgebra_glm as glm;
use omak::renderer::texture::{self, SpritesBuilder, Texture};
use omak::renderer::ImgKind;
use omak::{
    panels::{
        common::{GamePanel, Runnable},
        winit_panel::WindowWinit,
    },
    renderer::Renderer,
};
use std::path::Path;
use winit::event::VirtualKeyCode;

const TILE_SIZE: f32 = 32.0;
const TILES_IN_WIDTH: f32 = 26.0;
const TILES_IN_HEIGHT: f32 = 14.0;
const SCALE: f32 = 1.5;
const TILE_SIZE_SCALED: f32 = TILE_SIZE * SCALE;
const GAME_WIDTH: u32 = (TILE_SIZE_SCALED * TILES_IN_WIDTH) as u32;
const GAME_HEIGHT: u32 = (TILE_SIZE_SCALED * TILES_IN_HEIGHT) as u32;

//--------------------------------------------------------

fn main() {
    WindowWinit::build(GAME_WIDTH, GAME_HEIGHT).run(MyGame::new());
}

//--------------------------------------------------------

pub struct MyGame {
    player: Player,
    level_manager: LevelManager,
}

impl Runnable for MyGame {
    fn run(&mut self, panel: &mut impl GamePanel) {
        self.update(panel);
        self.draw(panel);
    }
}

impl MyGame {
    pub fn new() -> Self {
        Self {
            player: Player::new(
                300,
                200,
                64.0 * SCALE,
                40.0 * SCALE,
                "resources/img/player_sprites.png",
            ),
            level_manager: LevelManager::new(),
        }
    }

    fn update(&mut self, game_panel: &mut impl GamePanel) {
        self.player.update(game_panel);
    }

    fn draw(&mut self, game_panel: &mut impl GamePanel) {
        self.level_manager.draw(&mut game_panel.get_renderer());
        self.player.draw(&mut game_panel.get_renderer());
    }
}

//---------------------------------------------------------

pub struct Player {
    x: i32,
    y: i32,
    width: f32,
    height: f32,
    velocity: i32,
    animations_kind: AnimationsKind,
    animations: Vec<Texture>,
    animations_tick: i32,
    animations_index: usize,
    animations_speed: i32,
}
impl Player {
    pub fn new(x: i32, y: i32, width: f32, height: f32, image: &str) -> Self {
        let animations = SpritesBuilder::init(image, ImgKind::PNG)
            .with_rows(9, 64)
            .with_columns(6, 40)
            .build();

        Self {
            x,
            y,
            width,
            height,
            velocity: 3,
            animations_kind: AnimationsKind::Idle,
            animations,
            animations_tick: 0,
            animations_index: 0,
            animations_speed: 6,
        }
    }
    fn update(&mut self, game_panel: &mut impl GamePanel) {
        self.update_animations_tick();
        self.handle_keys_events(game_panel);
    }

    fn handle_keys_events(&mut self, game_panel: &mut impl GamePanel) {
        let keys = game_panel.get_keys();

        if keys.len() == 0 {
            if let AnimationsKind::Running | AnimationsKind::Attacking = self.animations_kind {
                self.animations_index = 0;
            }
            self.animations_kind = AnimationsKind::Idle;
        }

        if keys.contains(&VirtualKeyCode::Up) {
            self.animations_kind = AnimationsKind::Running;
            self.y -= self.velocity;
        }
        if keys.contains(&VirtualKeyCode::Down) {
            self.animations_kind = AnimationsKind::Running;
            self.y += self.velocity;
        }
        if keys.contains(&VirtualKeyCode::Left) {
            self.animations_kind = AnimationsKind::Running;
            self.x -= self.velocity;
        }
        if keys.contains(&VirtualKeyCode::Right) {
            self.animations_kind = AnimationsKind::Running;
            self.x += self.velocity;
        }
        if keys.contains(&VirtualKeyCode::Q) {
            self.animations_kind = AnimationsKind::Attacking;
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_image(
            glm::vec2(self.x as f32, self.y as f32),
            glm::vec2(self.width as f32, self.height as f32),
            0.0,
            glm::vec3(1.0, 1.0, 1.0),
            &self
                .animations
                .get(texture::get_index(
                    self.animations_index,
                    self.animations_kind.get_index_and_count().0,
                    6,
                ))
                .unwrap(),
        );
    }

    pub fn update_animations_tick(&mut self) {
        self.animations_tick += 1;
        if self.animations_tick >= self.animations_speed {
            self.animations_tick = 0;
            self.animations_index += 1;
            if self.animations_index >= self.animations_kind.get_index_and_count().1 {
                self.animations_index = 0;
            }
        }
    }
}

pub struct Level {
    level_data: Vec<u8>,
}
impl Level {
    //
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

    pub fn update(&mut self) {}

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

pub enum AnimationsKind {
    Running,
    Idle,
    Jumping,
    Falling,
    Ground,
    Hitting,
    Attacking,
    AttackingJump1,
    AttackingJump2,
}
impl AnimationsKind {
    fn get_index_and_count(&self) -> (usize, usize) {
        match self {
            Self::Idle => (0, 5),
            Self::Running => (1, 6),
            Self::Jumping => (2, 3),
            Self::Falling => (3, 1),
            Self::Ground => (4, 2),
            Self::Hitting => (5, 4),
            Self::Attacking => (6, 3),
            Self::AttackingJump1 => (7, 3),
            Self::AttackingJump2 => (8, 3),
        }
    }
}
