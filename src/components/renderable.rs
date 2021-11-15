use specs::{Component, VecStorage};

#[derive(Component, Copy, Clone, Debug)]
#[storage(VecStorage)]
pub struct Renderable {}
