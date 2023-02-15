use specs::{Component, VecStorage};

use crate::physx::Vector2;

#[derive(Component, Copy, Clone, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub vec: Vector2,
}
