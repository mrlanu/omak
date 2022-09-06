use nalgebra_glm as glm;
use omak::{
    panels::{
        common::{GamePanel, Runnable},
        winit_panel::WindowWinit,
    },
    renderer::Renderer,
};
use winit::event::VirtualKeyCode;

//--------------------------------------------------------

fn main() {
    WindowWinit::build(1280, 800).run(MyGame::new());
}

//--------------------------------------------------------

pub struct MyGame {
    player: Player,
}

impl Runnable for MyGame {
    fn init(&mut self, panel: &mut impl GamePanel) {
        self.player.load_textures(panel);
    }
    fn run(&mut self, panel: &mut impl GamePanel) {
        self.update(panel);
        self.draw(panel);
    }
}

impl MyGame {
    pub fn new() -> Self {
        Self {
            player: Player::new(300, 200, 128, 80, "resources/img/player_sprites.png"),
        }
    }

    fn update(&mut self, game_panel: &mut impl GamePanel) {
        self.player.update(game_panel);
    }

    fn draw(&mut self, game_panel: &mut impl GamePanel) {
        self.player.draw(&mut game_panel.get_renderer());
    }
}

//---------------------------------------------------------

pub struct Player {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    velocity: i32,
    image: String,
    animations_kind: AnimationsKind,
    animations: [[glm::UVec4; 6]; 9],
    animations_tick: i32,
    animations_index: usize,
    animations_speed: i32,
}
impl Player {
    pub fn new(x: i32, y: i32, width: i32, height: i32, image: &str) -> Self {
        let mut animations = load_animations();
        Self {
            x,
            y,
            width,
            height,
            velocity: 2,
            image: image.to_string(),
            animations_kind: AnimationsKind::Idle,
            animations,
            animations_tick: 0,
            animations_index: 0,
            animations_speed: 7,
        }
    }
    fn update(&mut self, game_panel: &mut impl GamePanel) {
        self.update_animations_tick();
        self.handle_keys_events(game_panel);
    }

    fn handle_keys_events(&mut self, game_panel: &mut impl GamePanel) {
        let keys = game_panel.get_keys();
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

        if !keys.contains(&VirtualKeyCode::Up)
            && !keys.contains(&VirtualKeyCode::Down)
            && !keys.contains(&VirtualKeyCode::Left)
            && !keys.contains(&VirtualKeyCode::Right)
            && keys.len() == 0
        {
            if let AnimationsKind::Running | AnimationsKind::Attacking = self.animations_kind {
                self.animations_index = 0;
            }
            self.animations_kind = AnimationsKind::Idle;
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_subimage(
            glm::vec2(self.x as f32, self.y as f32),
            glm::vec2(self.width as f32, self.height as f32),
            0.0,
            glm::vec3(1.0, 1.0, 1.0),
            &self.image,
            self.animations[self.animations_kind.get_index_and_count().0][self.animations_index],
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

    pub fn load_textures(&self, panel: &mut impl GamePanel) {}
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

fn load_animations() -> [[glm::UVec4; 6]; 9] {
    let mut animations = [[glm::vec4(0, 0, 0, 0); 6]; 9];
    for j in 0..animations.len() {
        for i in 0..animations[j].len() {
            animations[j][i] = glm::vec4(i as u32 * 64, j as u32 * 40, 64, 40);
        }
    }
    animations
}
