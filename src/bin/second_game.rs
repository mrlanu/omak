use nalgebra_glm as glm;
use omak::{
    panels::{
        common::{GamePanel, Runnable},
        winit_panel::WindowWinit,
    },
    renderer::Renderer,
};

fn main() {
    WindowWinit::build(600, 400).run(SecondGame);
}

struct SecondGame;

impl Runnable for SecondGame {
    fn run(&mut self, panel: &mut impl GamePanel) {
        panel.get_renderer().draw_image(
            glm::vec2(1700.0, 200.0),
            glm::vec2(96.0, 96.0),
            0.0,
            glm::vec3(1.0, 1.0, 1.0),
            "boy/boy_down_1.png",
        );
    }
}
