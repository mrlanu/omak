use std::sync::mpsc::Receiver;

use crate::renderer::Renderer;
use gl::types::*;
use glfw::{self, Glfw};
use glfw::{Action, Context, Key, Window, WindowEvent};

pub struct GamePanel {
    width: u32,
    height: u32,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    glfw: Glfw,
    pub renderer: Renderer,
}
impl GamePanel {
    pub fn new(width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        // #[cfg(target_os = "macos")]
        // glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(width, height, "Omak", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        // ---------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Self {
            width,
            height,
            window,
            events,
            glfw,
            renderer: Renderer::new(
                width as f32,
                height as f32,
                "resources/shaders/basic.shader",
            ),
        }
    }

    pub fn run(&mut self, runnable: &mut impl Runnable) {
        while !self.window.should_close() {
            self.process_events();

            //--------------------------

            self.renderer.clear();
            runnable.run(self);

            //--------------------------

            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::FramebufferSize(w, h) => unsafe { gl::Viewport(0, 0, w, h) },
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}

pub trait Runnable {
    fn run(&mut self, panel: &mut GamePanel);
}
