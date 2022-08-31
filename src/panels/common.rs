use crate::renderer::Renderer;

pub trait GamePanel {
    fn build(width: u32, height: u32) -> Self;
    fn run(self, runnable: impl Runnable + 'static);
    fn get_renderer(&mut self) -> &mut Renderer;
    fn get_keys(&self) -> &[bool];
}
pub trait Runnable {
    fn run(&mut self, panel: &mut impl GamePanel);
}
