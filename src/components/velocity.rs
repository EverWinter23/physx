use specs::{Component, VecStorage};

#[derive(Component, Copy, Clone, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
