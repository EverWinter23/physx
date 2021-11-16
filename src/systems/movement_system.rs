use specs::{
    Read,
    Join,
    System,
    ReadStorage,
    WriteStorage,
};

use crate::timer::Timer;
use crate::components::{
    Position,
    Velocity,
};

use crate::game::DeltaTime;

pub struct MovementSystem<'a> {
    pub timer: &'a mut Timer,
}

impl<'a> System<'a> for MovementSystem<'a> {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (delta, mut positions, velocities) = data;

        let delta = delta.0;
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.x = position.x + velocity.x as f32 * delta;
            position.y = position.y + velocity.y as f32 * delta;
        }
    }
}

