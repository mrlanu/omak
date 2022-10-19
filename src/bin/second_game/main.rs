mod components;
mod levels;
use components::*;
use levels::LevelManager;
use nalgebra_glm as glm;
use omak::panels::{
    common::{GamePanel, Runnable},
    winit_panel::WindowWinit,
};
use omak::renderer::texture::{self, SpritesBuilder};
use omak::renderer::ImgKind;
use specs::{Builder, Join, RunNow, System, World, WorldExt, WriteStorage};

use winit::event::VirtualKeyCode;

const TILE_SIZE: f32 = 32.0;
const TILES_IN_WIDTH: f32 = 26.0;
const TILES_IN_HEIGHT: f32 = 14.0;
const SCALE: f32 = 1.5;
const TILE_SIZE_SCALED: f32 = TILE_SIZE * SCALE;
const GAME_WIDTH: u32 = (TILE_SIZE_SCALED * TILES_IN_WIDTH) as u32;
const GAME_HEIGHT: u32 = (TILE_SIZE_SCALED * TILES_IN_HEIGHT) as u32;

//--------------------------------------------------------

fn main() {
    WindowWinit::build(GAME_WIDTH, GAME_HEIGHT).run(MyGame::new());
}

//--------------------------------------------------------

pub struct MyGame {
    ecs: World,
}

impl Runnable for MyGame {
    fn run(&mut self, panel: &mut impl GamePanel) {
        self.update(panel);
        self.draw(panel);
    }
}

impl MyGame {
    pub fn new() -> Self {
        Self { ecs: init_world() }
    }

    fn run_systems(&mut self) {
        let mut ani_tick = AnimationTick;
        ani_tick.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn update(&mut self, game_panel: &mut impl GamePanel) {
        self.run_systems();
        self.handle_keys_events(game_panel);
    }

    fn draw(&mut self, game_panel: &mut impl GamePanel) {
        let level_manager = self.ecs.fetch::<LevelManager>();
        level_manager.draw(&mut game_panel.get_renderer());

        let positions = self.ecs.read_storage::<Position>();
        let mut animations = self.ecs.write_storage::<Animation>();
        let dimentions = self.ecs.read_storage::<Dimension>();
        for (pos, dimention, animation) in (&positions, &dimentions, &mut animations).join() {
            game_panel.get_renderer().draw_image(
                glm::vec2(pos.x, pos.y),
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

    fn handle_keys_events(&mut self, game_panel: &mut impl GamePanel) {
        let keys = game_panel.get_keys();
        if keys.len() == 0 {
            self.set_animation(Actions::Idle);
        }

        if keys.contains(&VirtualKeyCode::Up) && !keys.contains(&VirtualKeyCode::Down) {
            self.move_player(Actions::MoveUp);
            self.set_animation(Actions::MoveUp);
        }
        if keys.contains(&VirtualKeyCode::Down) && !keys.contains(&VirtualKeyCode::Up) {
            self.move_player(Actions::MoveDown);
            self.set_animation(Actions::MoveDown);
        }
        if keys.contains(&VirtualKeyCode::Left) && !keys.contains(&VirtualKeyCode::Right) {
            self.move_player(Actions::MoveLeft);
            if keys.contains(&VirtualKeyCode::Q) {
                self.set_animation(Actions::Attacking);
            } else {
                self.set_animation(Actions::MoveLeft);
            }
        }
        if keys.contains(&VirtualKeyCode::Right) && !keys.contains(&VirtualKeyCode::Left) {
            self.move_player(Actions::MoveRight);
            if keys.contains(&VirtualKeyCode::Q) {
                self.set_animation(Actions::Attacking);
            } else {
                self.set_animation(Actions::MoveRight);
            }
        }
        if !keys.contains(&VirtualKeyCode::Right)
            && !keys.contains(&VirtualKeyCode::Left)
            && keys.contains(&VirtualKeyCode::Q)
        {
            self.set_animation(Actions::Attacking);
        }
    }

    fn set_animation(&mut self, action: Actions) {
        let mut animations = self.ecs.write_storage::<Animation>();
        for ani in (&mut animations).join() {
            let start_animation = ani.animations_kind;
            match action {
                Actions::MoveUp | Actions::MoveDown | Actions::MoveLeft | Actions::MoveRight => {
                    ani.animations_kind = AnimationsKind::Running
                }
                Actions::Attacking => ani.animations_kind = AnimationsKind::Attacking,
                _ => ani.animations_kind = AnimationsKind::Idle,
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

    fn move_player(&self, action: Actions) {
        let mut positions = self.ecs.write_storage::<Position>();
        let mut players = self.ecs.write_storage::<Player>();
        let mut coliders = self.ecs.write_storage::<Colider>();
        let velocities = self.ecs.read_storage::<Velocity>();
        let mut animations = self.ecs.write_storage::<Animation>();
        let level_manager = self.ecs.fetch::<LevelManager>();

        for (_player, pos, col, vel, ani) in (
            &mut players,
            &mut positions,
            &mut coliders,
            &velocities,
            &mut animations,
        )
            .join()
        {
            match action {
                Actions::Idle => {}
                Actions::MoveUp => {
                    if level_manager.can_move_here(
                        col.x,
                        col.y - vel.velocity,
                        col.width,
                        col.height,
                    ) {
                        pos.y -= vel.velocity;
                        col.y -= vel.velocity;
                    }
                }
                Actions::MoveDown => {
                    if level_manager.can_move_here(
                        col.x,
                        col.y + vel.velocity,
                        col.width,
                        col.height,
                    ) {
                        pos.y += vel.velocity;
                        col.y += vel.velocity;
                    }
                }
                Actions::MoveLeft => {
                    if level_manager.can_move_here(
                        col.x - vel.velocity,
                        col.y,
                        col.width,
                        col.height,
                    ) {
                        pos.x -= vel.velocity;
                        col.x -= vel.velocity;
                    }
                }
                Actions::MoveRight => {
                    if level_manager.can_move_here(
                        col.x + vel.velocity,
                        col.y,
                        col.width,
                        col.height,
                    ) {
                        pos.x += vel.velocity;
                        col.x += vel.velocity;
                    }
                }
                Actions::Attacking => {}
            }
        }
    }
}

fn init_world() -> World {
    let mut ecs = World::new();
    ecs.register::<Position>();
    ecs.register::<Player>();
    ecs.register::<Dimension>();
    ecs.register::<Velocity>();
    ecs.register::<Animation>();
    ecs.register::<Colider>();
    let level_manager = LevelManager::new();
    ecs.insert(level_manager);

    ecs.create_entity()
        .with(Player)
        .with(Position { x: 320.0, y: 338.0 })
        .with(Dimension {
            width: 64.0 * SCALE,
            height: 40.0 * SCALE,
        })
        .with(Velocity { velocity: 3.0 })
        .with(Animation {
            animations_kind: AnimationsKind::Idle,
            animations: SpritesBuilder::init("resources/img/player_sprites.png", ImgKind::PNG)
                .with_rows(9, 64)
                .with_columns(6, 40)
                .build(),

            animations_tick: 0,
            animations_index: 0,
            animations_speed: 6,
        })
        .with(Colider {
            x: 353.0,
            y: 346.0,
            width: 20.0 * SCALE,
            height: 25.0 * SCALE,
        })
        .build();
    ecs
}

struct AnimationTick;
impl<'a> System<'a> for AnimationTick {
    type SystemData = WriteStorage<'a, Animation>;
    fn run(&mut self, mut anims: Self::SystemData) {
        for ani in (&mut anims).join() {
            ani.animations_tick += 1;
            if ani.animations_tick >= ani.animations_speed {
                ani.animations_tick = 0;
                ani.animations_index += 1;
                if ani.animations_index >= ani.animations_kind.get_index_and_count().1 {
                    ani.animations_index = 0;
                }
            }
        }
    }
}

enum Actions {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Idle,
    Attacking,
}
