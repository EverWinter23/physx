use specs::{Component, VecStorage};

#[derive(Component, Copy, Clone, Debug)]
#[storage(VecStorage)]
pub struct Transform {
    pub radius: f32,
    pub scale: u32,
    pub r_width: f32,
    pub r_height: f32,
}
