use nalgebra_glm as glm;
use omak::{
    game_panel::{GamePanel, Runnable},
    renderer::Renderer,
};

fn main() {
    GamePanel::new(640, 400).run(&mut MyGame::new());
}

pub struct MyGame {
    player: Player,
}

impl Runnable for MyGame {
    fn run(&mut self, panel: &mut GamePanel) {
        self.update(panel);
        self.draw(panel);
    }
}

impl MyGame {
    pub fn new() -> Self {
        Self {
            player: Player::new(300, 200, 32, 32, "boy_down_1.png"),
        }
    }

    fn update(&mut self, game_panel: &mut GamePanel) {
        self.player.update(game_panel);
    }

    fn draw(&mut self, game_panel: &mut GamePanel) {
        self.player.draw(&mut game_panel.renderer);
    }
}

pub struct Player {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    image: String,
}
impl Player {
    pub fn new(x: i32, y: i32, width: i32, height: i32, image: &str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            image: image.to_string(),
        }
    }
    fn update(&mut self, game_panel: &mut GamePanel) {
        if game_panel.keys[glfw::Key::Up.get_scancode().unwrap() as usize] {
            self.y -= 5;
        }
        if game_panel.keys[glfw::Key::Down.get_scancode().unwrap() as usize] {
            self.y += 5;
        }
        if game_panel.keys[glfw::Key::Left.get_scancode().unwrap() as usize] {
            self.x -= 5;
        }
        if game_panel.keys[glfw::Key::Right.get_scancode().unwrap() as usize] {
            self.x += 5;
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) {
        renderer.draw_image(
            glm::vec2(self.x as f32, self.y as f32),
            glm::vec2(self.width as f32, self.height as f32),
            0.0,
            glm::vec3(1.0, 1.0, 1.0),
            &self.image,
        );
    }
}
