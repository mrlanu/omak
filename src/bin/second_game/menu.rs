use omak::panels::common::GamePanel;

pub struct Menu {
    pub touched: bool,
}
impl Menu {
    pub fn new() -> Self {
        Self { touched: true }
    }

    pub fn run(&mut self, panel: &mut impl GamePanel) {
        panel.get_renderer().println(550.0, 150.0, 32.0, "MENU");
    }
}
