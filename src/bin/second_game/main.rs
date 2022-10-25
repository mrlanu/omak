mod components;
mod levels;
mod menu;
mod play;
mod systems;
use components::*;
use levels::LevelManager;
use menu::Menu;
use omak::panels::{
    common::{GamePanel, Runnable},
    winit_panel::WindowWinit,
};
use omak::renderer::texture::SpritesBuilder;
use omak::renderer::ImgKind;
use play::Play;
use specs::{Builder, World, WorldExt};
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
    mode: GameMode,
    play: Play,
    menu: Menu,
}

impl Runnable for MyGame {
    fn run(&mut self, panel: &mut impl GamePanel) {
        match self.mode {
            GameMode::Menu => {
                if panel.get_keys().contains(&VirtualKeyCode::P) {
                    self.mode = GameMode::Playing;
                } else {
                    self.menu.run(panel);
                }
            }
            GameMode::Playing => {
                if panel.get_keys().contains(&VirtualKeyCode::M) {
                    self.mode = GameMode::Menu;
                }
                self.play.run(panel);
            }
            GameMode::End => {
                unimplemented!();
            }
        }
    }
}

impl MyGame {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Playing,
            play: Play::new(),
            menu: Menu::new(),
        }
    }
}

pub fn init_world() -> World {
    let mut ecs = World::new();
    ecs.register::<Position>();
    ecs.register::<Player>();
    ecs.register::<EntityState>();
    ecs.register::<Dimension>();
    ecs.register::<Velocity>();
    ecs.register::<Animation>();
    ecs.register::<Colider>();
    ecs.register::<Jump>();
    let level_manager = LevelManager::new();
    ecs.insert(level_manager);

    ecs.create_entity()
        .with(Player)
        .with(EntityState {
            moving: false,
            attacking: false,
            left: false,
            right: false,
            up: false,
            down: false,
            jump: false,
            in_air: false,
        })
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
        .with(Jump {
            air_speed: 0.0,
            gravity: 0.04 * SCALE,
            jump_speed: -2.25 * SCALE,
            fall_speed: 1.5 * SCALE,
        })
        .build();
    ecs
}

enum GameMode {
    Menu,
    Playing,
    End,
}
