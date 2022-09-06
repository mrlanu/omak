use std::collections::HashSet;
use winit::event::VirtualKeyCode as Key;

use crate::renderer::Renderer;

pub trait GamePanel {
    fn build(width: u32, height: u32) -> Self;
    fn run(self, runnable: impl Runnable + 'static);
    fn get_renderer(&mut self) -> &mut Renderer;
    fn get_keys(&self) -> &HashSet<Key>;
}
pub trait Runnable {
    fn init(&mut self, panel: &mut impl GamePanel); // will be called first
    fn run(&mut self, panel: &mut impl GamePanel);
}
