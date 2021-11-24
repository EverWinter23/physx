use crate::components::{Player, Velocity};

use specs::{Join, ReadStorage, System, WriteStorage};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct InputSystem<'a> {
    pub events: &'a mut EventPump,
}

impl<'a> System<'a> for InputSystem<'a> {
    type SystemData = (ReadStorage<'a, Player>, WriteStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let velocity = 50f32;

        let (_, mut velocities) = data;
        let mut players = (&mut velocities).join();
        let player_velocity = players.next().unwrap();
        for event in self.events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    player_velocity.y = -velocity;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    player_velocity.y = 0f32;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    player_velocity.y = velocity;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    player_velocity.y = 0f32;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    player_velocity.x = -velocity;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    player_velocity.x = 0f32;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    player_velocity.x = velocity;
                }
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    player_velocity.x = 0f32;
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
