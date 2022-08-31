use std::sync::mpsc::Receiver;

use crate::panels::common::{GamePanel, Runnable};
use crate::renderer::Renderer;
use gl::types::*;
use glfw::{self, Glfw};
use glfw::{Action, Context, Key, Window, WindowEvent};

pub struct WindowGlfw {
    pub width: u32,
    pub height: u32,
    window: Window,
    events: Receiver<(f64, WindowEvent)>,
    glfw: Glfw,
    pub keys: [bool; 1024],
    pub renderer: Renderer,
}
impl WindowGlfw {
    fn init(&mut self) {
        let projection: nalgebra_glm::Mat4 =
            nalgebra_glm::ortho(0.0, self.width as f32, self.height as f32, 0.0, -1.0, 1.0);
        let shader = self.renderer.res_manager.load_shader("sprite.shader");
        shader.activate();
        shader.set_uniform_1i("image", 0);
        shader.set_matrix4("projection", &projection);
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                WindowEvent::FramebufferSize(w, h) => unsafe { gl::Viewport(0, 0, w, h) },
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window.set_should_close(true)
                }
                WindowEvent::Key(_, KEY, Action::Press, _) => {
                    self.keys[KEY as usize] = true;
                }
                WindowEvent::Key(_, KEY, Action::Release, _) => {
                    self.keys[KEY as usize] = false;
                }
                _ => {}
            }
        }
    }
}

impl GamePanel for WindowGlfw {
    fn build(width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        // #[cfg(target_os = "macos")]
        // glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(width, height, "Omak", glfw::WindowMode::Windowed)
            .expect("Failed to build GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        // glfw.set_swap_interval(glfw::SwapInterval::None);
        // ---------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Self {
            width,
            height,
            window,
            events,
            glfw,
            keys: [false; 1024],
            renderer: Renderer::new(width as f32, height as f32),
        }
    }

    fn run(mut self, mut runnable: impl Runnable) {
        self.init();
        let mut prev_time = self.glfw.get_time();
        let mut frame_count = 0;

        while !self.window.should_close() {
            let current_time = self.glfw.get_time();
            frame_count += 1;
            if current_time - prev_time >= 1.0 {
                println!("FPS: {}", frame_count);
                frame_count = 0;
                prev_time = current_time;
            }
            self.process_events();

            //--------------------------

            self.renderer.clear();
            runnable.run(&mut self);

            //--------------------------

            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    fn get_renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    fn get_keys(&self) -> &[bool] {
        &self.keys[..]
    }
}
