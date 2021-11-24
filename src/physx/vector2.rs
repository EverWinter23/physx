use std::cmp;
use std::ops;

use sdl2::rect::Point;

#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0f32, y: 0f32 }
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, vec: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + vec.x,
            y: self.y + vec.y,
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, vec: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - vec.x,
            y: self.y - vec.y,
        }
    }
}

impl ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, scalar: f32) -> Vector2 {
        Vector2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl ops::Mul<Vector2> for f32 {
    type Output = Vector2;

    fn mul(self, vec: Vector2) -> Vector2 {
        Vector2 {
            x: self * vec.x,
            y: self * vec.y,
        }
    }
}

impl ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, scalar: f32) -> Vector2 {
        if scalar == 0f32 {
            panic!("[error]: cannot divide by 0!");
        }

        Vector2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl ops::Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl cmp::PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl cmp::Eq for Vector2 {}

impl From<Point> for Vector2 {
    fn from(point: Point) -> Vector2 {
        Vector2::new(point.x as f32, point.y as f32)
    }
}

impl Into<Point> for Vector2 {
    fn into(self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_addition() {
        let a_vec = Vector2::new(5f32, 6f32);
        let b_vec = Vector2::new(5f32, 4f32);
        let c_vec = a_vec + b_vec;
        assert_eq!(c_vec.x, 10f32);
        assert_eq!(c_vec.y, 10f32);
    }

    #[test]
    fn test_vec_negation() {
        let a_vec = Vector2::new(4f32, 6f32);
        let b_vec = -a_vec;
        assert_eq!(b_vec.x, -4f32);
        assert_eq!(b_vec.y, -6f32);
    }

    #[test]
    fn test_vector_multiplication_by_scalar() {
        let a_vec = Vector2::new(4f32, 6f32);
        let scalar = 2f32;
        let b_vec = a_vec * scalar;
        assert_eq!(b_vec.x, 8f32);
        assert_eq!(b_vec.y, 12f32);
    }

    #[test]
    fn test_scalar_multiplication_by_vector() {
        let a_vec = Vector2::new(4f32, 6f32);
        let scalar = 2f32;
        let b_vec = scalar * a_vec;
        assert_eq!(b_vec.x, 8f32);
        assert_eq!(b_vec.y, 12f32);
    }

    #[test]
    fn test_vector_division_by_scalar() {
        let a_vec = Vector2::new(4f32, 6f32);
        let scalar = 2f32;
        let b_vec = a_vec / scalar;
        assert_eq!(b_vec.x, 2f32);
        assert_eq!(b_vec.y, 3f32);
    }

    #[test]
    fn test_vector_equality() {
        let a_vec = Vector2::new(2f32, 3f32);
        let b_vec = Vector2::new(2f32, 3f32);
        let c_vec = Vector2::new(3f32, 2f32);

        assert_eq!(a_vec, b_vec);
        assert_ne!(a_vec, c_vec);
    }

    #[test]
    fn convert_vector_to_sdl_point() {
        let vec = Vector2::new(2f32, 3f32);
        let point: Point = vec.into();

        assert_eq!(point.x, 2);
        assert_eq!(point.y, 3);
    }

    #[test]
    fn convert_sdl_point_to_vector() {
        let point = Point::new(2, 3);
        let vec = Vector2::from(point);

        assert_eq!(vec.x, 2f32);
        assert_eq!(vec.y, 3f32);
    }
}
