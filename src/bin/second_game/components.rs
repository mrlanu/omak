use omak::renderer::texture::Texture;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Player;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Dimension {
    pub width: f32,
    pub height: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub velocity: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Animation {
    pub animations_kind: AnimationsKind,
    pub animations: Vec<Texture>,
    pub animations_tick: i32,
    pub animations_index: usize,
    pub animations_speed: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Colider {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationsKind {
    Running,
    Idle,
    Jumping,
    Falling,
    Ground,
    Hitting,
    Attacking,
    AttackingJump1,
    AttackingJump2,
}
impl AnimationsKind {
    pub fn get_index_and_count(&self) -> (usize, usize) {
        match self {
            Self::Idle => (0, 5),
            Self::Running => (1, 6),
            Self::Jumping => (2, 3),
            Self::Falling => (3, 1),
            Self::Ground => (4, 2),
            Self::Hitting => (5, 4),
            Self::Attacking => (6, 3),
            Self::AttackingJump1 => (7, 3),
            Self::AttackingJump2 => (8, 3),
        }
    }
}
