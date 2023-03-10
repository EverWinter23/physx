pub extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use specs::{Builder, RunNow, World, WorldExt};

use crate::components::{MouseEnd, Player, Position, Renderable, Transform, Velocity};
use crate::physx::Vector2;
use crate::systems::{CollisionSystem, InputSystem, MovementSystem, RenderingSystem};
use crate::timer::Timer;

#[derive(Default)]
pub struct DeltaTime(pub f32);

pub struct Game {
    world: World,
    canvas: WindowCanvas,
    events: EventPump,
    timer: Timer,
    pub is_running: bool,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let sdl_context = sdl2::init().expect("[error]: Could not initialize SDL.");

        let vid_subsytem = sdl_context
            .video()
            .expect("[error]: Could not initialize video subsytem.");

        let window = vid_subsytem
            .window("zelda.rs", width, height)
            .position_centered()
            .build()
            .expect("[error]: Could not create a window.");

        let canvas = window
            .into_canvas()
            .build()
            .expect("[error]: Could not create a canvas from a window.");

        let events = sdl_context
            .event_pump()
            .expect("[error]: Could not obtain an EventPump for SDL.");

        let timer = Timer::new(
            sdl_context
                .timer()
                .expect("[error]: Could not initialize timer."),
        );

        Game {
            world: World::new(),
            timer,
            canvas,
            events,
            is_running: true,
        }
    }

    pub fn init(&mut self) {
        // NOTE: register resources
        self.world.insert(DeltaTime(0.05));

        // NOTE: register components
        self.world.register::<MouseEnd>();
        self.world.register::<Player>();
        self.world.register::<Renderable>();
        self.world.register::<Position>();
        self.world.register::<Transform>();
        self.world.register::<Velocity>();

        self.world
            .create_entity()
            .with(Transform {
                r_width: 10f32,
                r_height: 10f32,
                radius: 10f32,
                scale: 1,
            })
            .with(MouseEnd {})
            .with(Renderable {})
            .with(Position {
                vec: Vector2::new(200f32, 200f32),
            })
            .with(Velocity {
                vec: Vector2::new(0f32, 0f32),
            })
            .build();

        // TODO: remove after verifying rendering system
        self.world
            .create_entity()
            .with(Transform {
                r_width: 10f32,
                r_height: 10f32,
                radius: 20f32,
                scale: 1,
            })
            .with(Renderable {})
            .with(Position {
                vec: Vector2::new(30f32, 30f32),
            })
            .with(Velocity {
                vec: Vector2::new(0f32, 0f32),
            })
            .build();

        self.world
            .create_entity()
            .with(Transform {
                r_width: 10f32,
                r_height: 10f32,
                radius: 20f32,
                scale: 1,
            })
            .with(Renderable {})
            .with(Position {
                vec: Vector2::new(150f32, 200f32),
            })
            .with(Velocity {
                vec: Vector2::new(0f32, 0f32),
            })
            .build();

        self.world
            .create_entity()
            .with(Transform {
                r_width: 10f32,
                r_height: 10f32,
                radius: 20f32,
                scale: 1,
            })
            .with(Player {})
            .with(Renderable {})
            .with(Position {
                vec: Vector2::new(150f32, 300f32),
            })
            .with(Velocity {
                vec: Vector2::new(0f32, 0f32),
            })
            .build();
    }

    pub fn update(&mut self) {
        let (tick, delta) = self.timer.tick();

        if !tick {
            return;
        } else {
            let mut dt = self.world.write_resource::<DeltaTime>();
            *dt = DeltaTime(delta);
        }

        let timer = &mut self.timer;
        {
            let mut movement_system = MovementSystem { timer };
            movement_system.run_now(&self.world)
        }

        {
            let mut collision_system = CollisionSystem { timer };
            collision_system.run_now(&self.world);
        }
    }

    pub fn render(&mut self) {
        let canvas = &mut self.canvas;
        canvas.set_draw_color(Color::GRAY);
        canvas.clear();
        {
            let mut rendering_system = RenderingSystem { canvas };
            rendering_system.run_now(&self.world);
        }
        canvas.present();
    }

    pub fn process_input(&mut self) {
        let events = &mut self.events;
        {
            let mut input_system = InputSystem { events };
            input_system.run_now(&self.world);
        }
    }
}
