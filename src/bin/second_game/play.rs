use crate::components::*;
use crate::levels::LevelManager;
use crate::systems::AnimationTick;
use nalgebra_glm as glm;
use omak::panels::common::GamePanel;
use omak::renderer::texture;
use specs::{Join, RunNow, World, WorldExt};
use winit::event::VirtualKeyCode;

pub struct Play {
    ecs: World,
}
impl Play {
    pub fn new() -> Self {
        Self {
            ecs: crate::init_world(),
        }
    }
    pub fn run(&mut self, panel: &mut impl GamePanel) {
        self.update(panel);
        self.draw(panel);
    }

    fn update(&mut self, game_panel: &mut impl GamePanel) {
        self.run_systems();
        self.handle_keys_events(game_panel);
        self.update_position();
        self.set_animation();
    }

    fn draw(&mut self, game_panel: &mut impl GamePanel) {
        let x_offset = 33.0;
        let y_offset = 8.0;
        let level_manager = self.ecs.fetch::<LevelManager>();
        level_manager.draw(&mut game_panel.get_renderer());

        let coliders = self.ecs.read_storage::<Colider>();
        let mut animations = self.ecs.write_storage::<Animation>();
        let dimentions = self.ecs.read_storage::<Dimension>();
        for (col, dimention, animation) in (&coliders, &dimentions, &mut animations).join() {
            game_panel.get_renderer().draw_image(
                glm::vec2(col.x - x_offset, col.y - y_offset),
                glm::vec2(dimention.width, dimention.height),
                0.0,
                glm::vec3(1.0, 1.0, 1.0),
                animation
                    .animations
                    .get(texture::get_index(
                        animation.animations_index,
                        animation.animations_kind.get_index_and_count().0,
                        6,
                    ))
                    .unwrap(),
            );
        }
    }

    fn run_systems(&mut self) {
        let mut ani_tick = AnimationTick;
        ani_tick.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn handle_keys_events(&mut self, game_panel: &mut impl GamePanel) {
        let keys = game_panel.get_keys();
        let players = self.ecs.read_storage::<Player>();
        let mut players_state = self.ecs.write_storage::<EntityState>();

        for (_player, st) in (&players, &mut players_state).join() {
            if keys.contains(&VirtualKeyCode::Left) {
                st.left = true;
            } else {
                st.left = false;
            }
            if keys.contains(&VirtualKeyCode::Right) {
                st.right = true;
            } else {
                st.right = false;
            }
            if keys.contains(&VirtualKeyCode::Space) {
                st.jump = true;
            } else {
                st.jump = false;
            }
            if keys.contains(&VirtualKeyCode::Q) {
                st.attacking = true;
            } else {
                st.attacking = false;
            }
        }
    }

    fn update_position(&self) {
        let mut players = self.ecs.write_storage::<Player>();
        let mut state = self.ecs.write_storage::<EntityState>();
        let mut coliders = self.ecs.write_storage::<Colider>();
        let velocities = self.ecs.read_storage::<Velocity>();
        let mut jumps = self.ecs.write_storage::<Jump>();

        let level_manager = self.ecs.fetch::<LevelManager>();

        for (_player, st, col, vel, jmp) in (
            &mut players,
            &mut state,
            &mut coliders,
            &velocities,
            &mut jumps,
        )
            .join()
        {
            st.moving = false;

            if st.jump {
                self.jump(st, jmp);
            }

            if !st.left && !st.right && !st.in_air {
                return;
            }

            let mut x_speed = 0.0;

            if st.left {
                x_speed -= vel.velocity;
            }
            if st.right {
                x_speed += vel.velocity;
            }

            if !st.in_air {
                if !level_manager.is_on_floor(col.x, col.y, col.width, col.height) {
                    st.in_air = true;
                }
            }

            if st.in_air {
                if level_manager.can_move_here(col.x, col.y + jmp.air_speed, col.width, col.height)
                {
                    col.y += jmp.air_speed;
                    jmp.air_speed += jmp.gravity;
                    self.update_x_position(&level_manager, col, x_speed);
                } else {
                    if jmp.air_speed > 0.0 {
                        self.reset_in_air(st, jmp);
                    } else {
                        jmp.air_speed = jmp.fall_speed;
                    }
                    self.update_x_position(&level_manager, col, x_speed);
                }
            } else {
                self.update_x_position(&level_manager, col, x_speed);
            }

            st.moving = true;
        }
    }

    fn set_animation(&mut self) {
        let players = self.ecs.read_storage::<Player>();
        let mut animations = self.ecs.write_storage::<Animation>();
        let mut states = self.ecs.write_storage::<EntityState>();
        let mut jumps = self.ecs.write_storage::<Jump>();

        for (_player, ani, st, jmp) in (&players, &mut animations, &mut states, &mut jumps).join() {
            let start_animation = ani.animations_kind;
            if st.moving {
                ani.animations_kind = AnimationsKind::Running;
            } else {
                ani.animations_kind = AnimationsKind::Idle;
            }

            if st.in_air {
                if jmp.air_speed < 0.0 {
                    ani.animations_kind = AnimationsKind::Jumping;
                } else {
                    ani.animations_kind = AnimationsKind::Falling;
                }
            }

            if st.attacking {
                ani.animations_kind = AnimationsKind::Attacking;
            }

            if start_animation != ani.animations_kind {
                self.reset_animation_tick(ani);
            }
        }
    }

    fn reset_animation_tick(&self, ani: &mut Animation) {
        ani.animations_tick = 0;
        ani.animations_index = 0;
    }

    fn update_x_position(&self, lvl_manager: &LevelManager, col: &mut Colider, x_speed: f32) {
        if lvl_manager.can_move_here(col.x + x_speed, col.y, col.width, col.height) {
            col.x += x_speed;
        }
    }

    fn jump(&self, st: &mut EntityState, jmp: &mut Jump) {
        if st.in_air {
            return;
        }
        st.in_air = true;
        jmp.air_speed = jmp.jump_speed;
    }

    fn reset_in_air(&self, st: &mut EntityState, jmp: &mut Jump) {
        st.in_air = false;
        jmp.air_speed = 0.0;
    }
}

enum Actions {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Idle,
    Attacking,
    Jump,
    Fall,
}
