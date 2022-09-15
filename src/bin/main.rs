mod ecs;
mod levels;
use ecs::*;
use nalgebra_glm as glm;
use omak::renderer::texture::{self, SpritesBuilder, Texture};
use omak::renderer::ImgKind;
use omak::{
    panels::{
        common::{GamePanel, Runnable},
        winit_panel::WindowWinit,
    },
    renderer::Renderer,
};
use specs::{Builder, Join, RunNow, System, World, WorldExt, WriteStorage};

use levels::LevelManager;
use winit::event::VirtualKeyCode;

const TILE_SIZE: f32 = 32.0;
pub const TILES_IN_WIDTH: f32 = 26.0;
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
        let mut level_manager = self.ecs.fetch::<LevelManager>();
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
            self.move_player(Actions::Idle);
        }

        if keys.contains(&VirtualKeyCode::Up) {
            self.move_player(Actions::MoveUp)
        }
        if keys.contains(&VirtualKeyCode::Down) {
            self.move_player(Actions::MoveDown)
        }
        if keys.contains(&VirtualKeyCode::Left) {
            self.move_player(Actions::MoveLeft)
        }
        if keys.contains(&VirtualKeyCode::Right) {
            self.move_player(Actions::MoveRight)
        }
        if keys.contains(&VirtualKeyCode::Q) {
            self.move_player(Actions::Attacking)
        }
    }

    fn move_player(&self, action: Actions) {
        let mut positions = self.ecs.write_storage::<Position>();
        let mut players = self.ecs.write_storage::<Player>();
        let velocities = self.ecs.read_storage::<Velocity>();
        let mut animations = self.ecs.write_storage::<Animation>();

        for (_player, pos, vel, ani) in
            (&mut players, &mut positions, &velocities, &mut animations).join()
        {
            match action {
                Actions::Idle => {
                    if let AnimationsKind::Running | AnimationsKind::Attacking = ani.animations_kind
                    {
                        ani.animations_index = 0;
                    }
                    ani.animations_kind = AnimationsKind::Idle;
                }
                Actions::MoveUp => {
                    ani.animations_kind = AnimationsKind::Running;
                    pos.y -= vel.velocity;
                }
                Actions::MoveDown => {
                    ani.animations_kind = AnimationsKind::Running;
                    pos.y += vel.velocity;
                }
                Actions::MoveLeft => {
                    ani.animations_kind = AnimationsKind::Running;
                    pos.x -= vel.velocity;
                }
                Actions::MoveRight => {
                    ani.animations_kind = AnimationsKind::Running;
                    pos.x += vel.velocity;
                }
                Actions::Attacking => {
                    ani.animations_kind = AnimationsKind::Attacking;
                }
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
    let level_manager = LevelManager::new();
    ecs.insert(level_manager);

    ecs.create_entity()
        .with(Player)
        .with(Position { x: 300.0, y: 200.0 })
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

pub enum Actions {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Idle,
    Attacking,
}
