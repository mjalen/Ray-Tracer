// use crate::shapes::plane::*;
use crate::shapes::sphere::*;
use crate::util::hittable::Hittable;

use super::plane::Plane;

// Wrapper shape type so the world has no need for dyn Hittable
// TODO Implement Plane shape.
#[derive(Clone, Copy)]
pub enum Shape {
    Sphere(Sphere),
    Plane(Plane)
}

impl Shape {
    pub fn sphere(obj: Sphere) -> Self {
        Shape::Sphere(obj)
    }

    pub fn plane(obj: Plane) -> Self {
        Shape::Plane(obj)
    }
}

impl Hittable for Shape {
    fn hit(&self, ray: crate::math::ray::Ray) -> Option<crate::util::hittable::RayCollision> {
        match self {
            Shape::Sphere(o) => o.hit(ray),
            Shape::Plane(o) => o.hit(ray)
        }
    }
}