use crate::physx;
use physx::Vector2;

pub fn length(vec: Vector2) -> f32 {
    (vec.x * vec.x + vec.y * vec.y).sqrt()
}

pub fn distance(a: Vector2, b: Vector2) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

pub fn normalize(vec: Vector2) -> Vector2 {
    let vec_len = length(vec);
    Vector2 {
        x: vec.x / vec_len,
        y: vec.y / vec_len,
    }
}

pub fn dot_product(a: Vector2, b: Vector2) -> f32 {
    a.x * b.x + a.y * b.y
}

pub fn cross_product(a: Vector2, b: Vector2) -> f32 {
    a.x * b.y - a.y * b.x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        let vec = Vector2::new(4f32, 3f32);
        assert_eq!(5f32, length(vec));
    }

    // TODO: Write tests for other functions
}
