use nalgebra_glm as glm;
use omak::{
    renderer::Renderer,
    winit_panel::{GamePanel, Gammable},
};

fn main() {
    let mut ok = GamePanel::new(600.0, 400.0).run(SecondGame);
}

struct SecondGame;

impl Gammable for SecondGame {
    fn run(&mut self, renderer: &mut Renderer) {
        renderer.draw_image(
            glm::vec2(1700.0, 200.0),
            glm::vec2(96.0, 96.0),
            0.0,
            glm::vec3(1.0, 1.0, 1.0),
            "boy/boy_down_1.png",
        );
    }
}
