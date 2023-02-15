use crate::components::{Player, Position, Velocity, MouseEnd};

use specs::{Join, ReadStorage, System, WriteStorage};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct InputSystem<'a> {
    pub events: &'a mut EventPump,
}

impl<'a> System<'a> for InputSystem<'a> {
    type SystemData = (
        // ReadStorage<'a, Player>,
        ReadStorage<'a, MouseEnd>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mouse_ends, mut velocities, mut positions) = data;

        let mut mouse_end = (&mouse_ends, &mut positions).join().next().unwrap();

        for event in self.events.poll_iter() {
            match event {
                Event::MouseMotion { x, y, .. } => {
                    mouse_end.1.vec.x = x as f32;
                    mouse_end.1.vec.y = y as f32;
                }
                Event::Quit { .. } => {}
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    // TODO: Make game state a resource
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }
}
