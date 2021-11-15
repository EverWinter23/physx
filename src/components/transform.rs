use specs::{Component, VecStorage};

#[derive(Component, Copy, Clone, Debug)]
#[storage(VecStorage)]
pub struct Transform {
    pub scale: u32,
    pub r_width: u32,
    pub r_height: u32,
}
