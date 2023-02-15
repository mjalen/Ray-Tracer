use crate::util::vector::Vector3;

pub type Point = Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Point
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: f32) -> Point {
        self.origin + self.direction.scalar_mul(t)
    }
}
