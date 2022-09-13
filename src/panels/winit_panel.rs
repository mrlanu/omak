use crate::panels::common::{GamePanel, Runnable};
use crate::renderer::Renderer;
use gl::types::*;
use glutin::dpi::PhysicalPosition;
use std::collections::HashSet;
use std::time::{Duration, Instant, SystemTime};
use winit::{
    dpi,
    event::{ElementState, Event, VirtualKeyCode as Key, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const FPS: f64 = 60.0;

pub struct WindowWinit {
    event_loop: Option<EventLoop<()>>,
    ctx: glutin::ContextWrapper<glutin::PossiblyCurrent, Window>,
    renderer: Renderer,
    keys: HashSet<Key>,
    time_created: Instant,
}

impl WindowWinit {
    fn user_input(&mut self, event: &Event<()>) {
        match event {
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(keycode) = input.virtual_keycode {
                        match input.state {
                            ElementState::Pressed => {
                                // self.keys.clear();
                                self.keys.insert(keycode);                           }
                            ElementState::Released => {
                                self.keys.remove(&keycode);                           }
                        }
                    }
                }
                _ => {}
            },

            Event::DeviceEvent {
                // event: DeviceEvent::MouseMotion { delta: (dx, dy) },
                ..
            } => {
                // self.mouse_delta = (*dx as f32, *dy as f32);
            }

            _ => {}
        }
    }
}

impl GamePanel for WindowWinit {
    fn build(width: u32, height: u32) -> Self {
        let window_builder = WindowBuilder::new()
            .with_title("Omak")
            .with_inner_size(dpi::PhysicalSize::new(width, height))
            .with_resizable(false)
            .with_position(PhysicalPosition::new(550, 250));
        let event_loop = EventLoop::new();
        unsafe {
            let ctx = glutin::ContextBuilder::new()
                .with_vsync(false)
                .build_windowed(window_builder, &event_loop)
                .unwrap();
            let ctx = ctx.make_current().unwrap();
            gl::load_with(|symbol| ctx.get_proc_address(symbol) as *const _);
            let window_size = ctx.window().inner_size();
            let monitor_size = ctx.window().current_monitor().unwrap().size();

            // center the window
            ctx.window().set_outer_position(PhysicalPosition::new(
                (monitor_size.width - window_size.width) / 2,
                (monitor_size.height - window_size.height) / 2,
            ));

            gl::Viewport(0, 0, window_size.width as i32, window_size.height as i32);
            Self {
                ctx,
                event_loop: Some(event_loop),
                renderer: Renderer::new(window_size.width, window_size.height),
                keys: HashSet::new(),
                time_created: Instant::now(),
            }
        }
    }

    fn run(mut self, mut runnable: impl Runnable + 'static) {
        let event_loop = self.event_loop.unwrap();
        self.event_loop = None;

        let time_per_frame = 1000000000.0 / FPS;
        let mut prev_time = self.time_created.elapsed().as_nanos();
        let mut frame_count = 0;
        let mut last_check = self.time_created.elapsed().as_nanos();
        let mut delta = 0.0;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
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
                    let current_time = self.time_created.elapsed().as_nanos();

                    delta += (current_time - prev_time) as f64 / time_per_frame;
                    prev_time = current_time;

                    if delta >= 1.0 {
                        self.renderer.clear();
                        runnable.run(&mut self);
                        self.ctx.swap_buffers().unwrap();
                        delta -= 1.0;

                        //should be deleted when textures loading delay will be fixed
                        if delta > 2.0 {
                            delta = 0.0;
                        }
                        //----------

                        frame_count += 1;
                    }

                    if current_time - last_check >= 1000000000 {
                        last_check = current_time;
                        // println!("FPS: {}", frame_count);
                        frame_count = 0;
                    }
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
                        Some(Key::Escape) => {
                            *control_flow = ControlFlow::Exit;
                        }
                        _ => {}
                    },
                    _ => (),
                },
                _ => (),
            }
            self.user_input(&event);
        });
    }

    fn get_renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    fn get_keys(&self) -> &HashSet<Key> {
        &self.keys
    }
}
