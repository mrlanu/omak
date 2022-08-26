use omak::game_panel::{GamePanel, Runnable};
use omak::renderer::texture::Texture;
use omak::renderer::utils::ResourcesManager;
use omak::renderer::{ImgKind, Renderer};

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
            player: Player::new(300, 200, 40, 40, "my_s.png"),
        }
    }

    fn update(&self) {}

    fn draw(&mut self, game_panel: &mut GamePanel) {
        // game_panel.renderer.draw_image(
        //     self.player.x as f32,
        //     self.player.y as f32,
        //     self.player.width as f32,
        //     self.player.height as f32,
        //     &self.player.image_path,
        //     ImgKind::PNG,
        // )
        let projection: nalgebra_glm::Mat4 = nalgebra_glm::ortho(
            0.0,
            game_panel.width as f32,
            game_panel.height as f32,
            0.0,
            -1.0,
            1.0,
        );
        let mut shader = game_panel.renderer.res_manager.load_shader("sprite.shader");
        shader.set_uniform_1i("image", 0);
        shader.set_matrix4("projection", &projection);
        game_panel.renderer.draw_sprite(
            nalgebra_glm::vec2(200.0, 200.0),
            nalgebra_glm::vec2(40.0, 40.0),
            0.0,
            nalgebra_glm::vec3(1.0, 1.0, 1.0),
        );
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
