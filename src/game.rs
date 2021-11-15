extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;

pub struct Game {
    canvas: WindowCanvas,
    events: EventPump,
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

        Game {
            canvas,
            events,
            is_running: true,
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(30, 60, 90));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn process_input(&mut self) {
        for event in self.events.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    self.is_running = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.is_running = false;
                }
                _ => {}
            }
        }
    }
}
