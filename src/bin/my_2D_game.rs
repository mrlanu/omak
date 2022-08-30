use nalgebra_glm as glm;
use omak::{
    game_panel::{GamePanel, Runnable, WindowGlfw},
    renderer::Renderer,
};

fn main() {
    WindowGlfw::build(640, 400).run(&mut MyGame::new());
}

//--------------------------------------------------------

pub struct MyGame {
    player: Player,
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
            player: Player::new(300, 200, 32, 32, "boy_down_1.png"),
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
    sprite_counter: i32,
    sprite_num: i32,
    image: String,
}
impl Player {
    pub fn new(x: i32, y: i32, width: i32, height: i32, image: &str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            velocity: 2,
            sprite_counter: 0,
            sprite_num: 0,
            image: image.to_string(),
        }
    }
    fn update(&mut self, game_panel: &mut impl GamePanel) {
        self.sprite_counter += 1;
        if self.sprite_counter > 10 {
            if self.sprite_num == 1 {
                self.sprite_num = 2;
            } else {
                self.sprite_num = 1;
            }
            self.sprite_counter = 0;
        }
        self.handle_keys_events(game_panel);
    }

    fn handle_keys_events(&mut self, game_panel: &mut impl GamePanel) {
        if game_panel.get_keys()[glfw::Key::Up.get_scancode().unwrap() as usize] {
            if self.sprite_num == 1 {
                self.image = "boy/boy_up_1.png".to_string();
            } else {
                self.image = "boy/boy_up_2.png".to_string();
            }
            self.y -= self.velocity;
        }
        if game_panel.get_keys()[glfw::Key::Down.get_scancode().unwrap() as usize] {
            if self.sprite_num == 1 {
                self.image = "boy/boy_down_1.png".to_string();
            } else {
                self.image = "boy/boy_down_2.png".to_string();
            }
            self.y += self.velocity;
        }
        if game_panel.get_keys()[glfw::Key::Left.get_scancode().unwrap() as usize] {
            if self.sprite_num == 1 {
                self.image = "boy/boy_left_1.png".to_string();
            } else {
                self.image = "boy/boy_left_2.png".to_string();
            }
            self.x -= self.velocity;
        }
        if game_panel.get_keys()[glfw::Key::Right.get_scancode().unwrap() as usize] {
            if self.sprite_num == 1 {
                self.image = "boy/boy_right_1.png".to_string();
            } else {
                self.image = "boy/boy_right_2.png".to_string();
            }

            self.x += self.velocity;
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
