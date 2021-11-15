use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use specs::{Join, ReadStorage, System};

use crate::components::{Position, Renderable, Transform};

pub struct RenderingSystem<'a> {
    pub canvas: &'a mut WindowCanvas,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_, positions, transforms) = data;
        let entities = (&positions, &transforms).join().collect::<Vec<_>>();

        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for (position, transform) in entities.iter() {
            self.canvas
                .fill_rect(Rect::new(
                    position.x as i32,
                    position.y as i32,
                    transform.r_width * transform.scale as u32,
                    transform.r_height * transform.scale as u32,
                ))
                .expect("[error]: Could not fill rectangle!");
        }
        self.canvas.present();
    }
}
