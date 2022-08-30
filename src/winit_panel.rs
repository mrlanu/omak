use crate::renderer::Renderer;
use gl::types::*;
use winit::{
    dpi,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{self as winit_window, Window, WindowBuilder},
};

pub struct GamePanel {
    pub event_loop: EventLoop<()>,
    pub ctx: glutin::ContextWrapper<glutin::PossiblyCurrent, winit_window::Window>,
    pub renderer: Renderer,
}

impl GamePanel {
    pub fn new(width: f32, height: f32) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title("Omak")
            .with_inner_size(dpi::LogicalSize::new(width, height));
        let event_loop = EventLoop::new();
        unsafe {
            let ctx = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, &event_loop)
                .unwrap();
            let ctx = ctx.make_current().unwrap();
            gl::load_with(|symbol| ctx.get_proc_address(symbol) as *const _);
            unsafe {
                let window_size = ctx.window().inner_size();
                gl::Viewport(0, 0, window_size.width as i32, window_size.height as i32);
            }

            Self {
                ctx,
                event_loop,
                renderer: Renderer::new(width, height),
            }
        }
    }

    fn init(&mut self) {
        let window_size = self.ctx.window().inner_size();
        let projection: nalgebra_glm::Mat4 = nalgebra_glm::ortho(
            0.0,
            window_size.width as f32,
            window_size.height as f32,
            0.0,
            -1.0,
            1.0,
        );
        let shader = self.renderer.res_manager.load_shader("sprite.shader");
        shader.activate();
        shader.set_uniform_1i("image", 0);
        shader.set_matrix4("projection", &projection);
    }

    pub fn run(mut self, mut gamable: impl Gammable + 'static) {
        self.init();

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => {
                    return;
                }

                // "Emitted when all of the event loop's input events have been processed
                // and redraw processing is about to begin"
                Event::MainEventsCleared => {
                    self.ctx.window().request_redraw();
                }

                // Draw to the screen when requested
                Event::RedrawRequested(_) => {
                    self.renderer.clear();
                    gamable.run(&mut self.renderer);
                    self.ctx.swap_buffers().unwrap();
                }

                Event::WindowEvent { ref event, .. } => match event {
                    // Resize OpenGL viewport when window is resized
                    WindowEvent::Resized(size) => unsafe {
                        gl::Viewport(0, 0, size.width as i32, size.height as i32);
                    },

                    // Exit loop when CloseRequested raised
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                        // Exit loop when Escape is pressed
                        Some(VirtualKeyCode::Escape) => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    },
                    _ => (),
                },
                _ => (),
            }
        });
    }
}

pub trait Gammable {
    fn run(&mut self, panel: &mut Renderer);
}
