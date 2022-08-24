use omak::game_panel::{GamePanel, Runnable};
use omak::renderer::ImgKind;

fn main() {
    GamePanel::new(640, 400).run(&mut MyGame::new());
}

pub struct MyGame {
    player: Player,
}

impl Runnable for MyGame {
    fn run(&mut self, panel: &mut GamePanel) {
        self.update();
        self.draw(panel);
    }
}

impl MyGame {
    pub fn new() -> Self {
        Self {
            player: Player::new(300, 200, 16, 16, "resources/img/boy_down_1.png"),
        }
    }

    fn update(&self) {}

    fn draw(&mut self, game_panel: &mut GamePanel) {
        game_panel.renderer.draw_image(
            self.player.x as f32,
            self.player.y as f32,
            self.player.width as f32,
            self.player.height as f32,
            &self.player.image_path,
            ImgKind::PNG,
        )
    }
}

pub struct Player {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    image_path: String,
}
impl Player {
    pub fn new(x: i32, y: i32, width: i32, height: i32, image_path: &str) -> Self {
        Self {
            x,
            y,
            width,
            height,
            image_path: image_path.to_string(),
        }
    }
}
