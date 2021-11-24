use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use specs::{Join, ReadStorage, System};

use crate::components::{Position, Renderable, Transform};
use crate::game::sdl2::gfx::primitives::DrawRenderer;

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
        self.canvas.set_draw_color(Color::WHITE);
        for (position, transform) in entities.iter() {
            self.canvas
                .filled_circle(
                    position.x as i16,
                    position.y as i16,
                    transform.radius as i16,
                    Color::WHITE,
                )
                .expect("[error]: Could not fill rectangle!");
        }
        self.canvas.present();
    }
}
