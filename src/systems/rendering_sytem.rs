use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::render::WindowCanvas;
use specs::{Join, ReadStorage, System};


use crate::components::{Position, Renderable, Transform, Player, MouseEnd};
use crate::game::sdl2::gfx::primitives::DrawRenderer;

pub struct RenderingSystem<'a> {
    pub canvas: &'a mut WindowCanvas,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, MouseEnd>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_, players, mouse_ends, positions, transforms) = data;
        let entities = (&positions, &transforms).join().collect::<Vec<_>>();

        self.canvas.clear();
        self.canvas.set_draw_color(Color::WHITE);
        for (position, transform) in entities.iter() {
            self.canvas
                .circle(
                    position.vec.x as i16,
                    position.vec.y as i16,
                    transform.radius as i16,
                    Color::WHITE,
                )
                .expect("[error]: Could not fill rectangle!");
        }

        let mouse_end = (&mouse_ends, &positions).join().next().unwrap();
        let player = (&players, &positions).join().next().unwrap();

        self.canvas
            .draw_line(Point::new(mouse_end.1.vec.x as i32, mouse_end.1.vec.y as i32), Point::new(player.1.vec.x as i32, player.1.vec.y as i32))
            .expect("[error]: Could not draw the stick of truth!");

        self.canvas
            .draw_rect(Rect::new(mouse_end.1.vec.x as i32, mouse_end.1.vec.y as i32, player.1.vec.x as u32, player.1.vec.y as u32))
            .expect("[error]: Can't draw shtick");

        self.canvas.present();
    }
}
