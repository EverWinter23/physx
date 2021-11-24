use specs::{Join, Read, ReadStorage, System, WriteStorage};

use crate::components::{Position, Velocity};
use crate::timer::Timer;

use crate::game::DeltaTime;

pub struct MovementSystem<'a> {
    pub timer: &'a mut Timer,
}

impl<'a> System<'a> for MovementSystem<'a> {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, mut positions, velocities) = data;

        let delta = delta.0;
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.vec.x = position.vec.x + velocity.vec.x as f32 * delta;
            position.vec.y = position.vec.y + velocity.vec.y as f32 * delta;
        }
    }
}
