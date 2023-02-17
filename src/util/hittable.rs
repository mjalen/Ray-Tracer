use crate::math::vector::Vector3;
use crate::math::ray::Ray;

type Point = Vector3;

#[derive(Copy, Clone, Debug)]
pub struct RayCollision { // returned when an object is hit by a ray.
    pub hit_point: Point,
    pub normal: Point,
    pub distance: f32
}

impl RayCollision {
    pub fn new(hit_point: Point, normal: Point, distance: f32) -> Self {
        RayCollision { hit_point, normal, distance }
    }
}

// trait for all render objects
pub trait Hittable {
    fn hit(self, ray: Ray) -> Option<RayCollision>;
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(self, ray: Ray) -> Option<RayCollision> {
        // t^2*b*b + 2tb*(A-C)+(A-C)*(A-C) t solved using quadratic formula
        let o_min_c: Point = ray.origin + self.center.scalar_mul(-1.0);
        let a: f32 = ray.direction.dot(&ray.direction); 
        let b: f32 = 2.0 * ray.direction.dot(&o_min_c);
        let c: f32 = o_min_c.dot(&o_min_c) - self.radius * self.radius;
        let formula_sqrt: f32 = b * b - 4.0 * a * c;
        if formula_sqrt < 0.0 {
            return None;
        }

        // finish quad formula to get ray distance. 
        let dist: f32 = (-1.0 * b - formula_sqrt.sqrt()) / (2.0 * a);
        let collision: Point = ray.at(dist);
        let norm: Point = (collision + self.center.scalar_mul(-1.0)).unit(); 
        
        Some(RayCollision::new(collision, norm, dist)) 

    }
}


