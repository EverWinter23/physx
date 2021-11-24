use specs::{
    Join,
    ReadStorage,
    WriteStorage,
    System,
};

use crate::components::{Transform, Position};
use crate::timer::Timer;

use crate::physx::Vector2;

use crate::physx::math;

pub struct CollisionSystem<'a> {
    pub timer: &'a Timer,
}

pub fn are_circles_colliding(r1: f32, r2: f32, p1: Vector2, p2: Vector2) -> (bool, Vector2, f32) {
   let d = r1 + r2;
   let x = math::distance(p1, p2);

   if x > d {
       return (false, Vector2::zero(), 0f32);
   } 

   (true, math::normalize(p2 - p1), d - x)
}

impl<'a> System<'a> for CollisionSystem<'a> {
    type SystemData = (ReadStorage<'a, Transform>, WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (transforms, mut positions) = data;
        let mut bodies = (&transforms, &mut positions).join().collect::<Vec<_>>();

        for i in 0..bodies.len() - 1 {
            for j in i + 1..bodies.len() {
                let (t1, p1) = &bodies[i];
                let (t2, p2) = &bodies[j];
                let (collision, normal, depth) =  are_circles_colliding(
                    t1.radius, t2.radius,
                    Vector2::new(p1.vec.x, p1.vec.y),
                    Vector2::new(p2.vec.x, p2.vec.y)
                );

                if collision {
                    let b1 = bodies.get_mut(i).unwrap();
                    b1.1.vec = b1.1.vec - normal * depth / 2f32;

                    let b2 = bodies.get_mut(j).unwrap();
                    b2.1.vec = b2.1.vec + normal * depth / 2f32;
                }
            }
        }
    }
}
