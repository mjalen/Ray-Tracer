use crate::util::material::Material;
use crate::{math::vector::*, util::hittable::RayCollision};
use crate::math::ray::Ray;
use crate::util::hittable::*;

#[derive(Copy, Clone, Debug)]
pub struct Plane {
    pub p1: Point,
    pub p2: Point,
    pub n: Point,
    pub material: Material
}

impl Plane {
    pub fn new (p1: Point, p2: Point, n: Point, material: Material) -> Plane {
        let p = Plane { p1, p2, n, material };

        print!("Plane: {:?}", p);
        p
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: Ray) -> Option<RayCollision> {
        let t_min: f32 = 0.001; 
        let t_max: f32 = f32::INFINITY;

        let t = (self.p1.dot(self.n) - ray.origin.dot(self.n)) / (ray.direction.dot(self.n));

        if t < t_min || t > t_max {
            return None
        }

        // auto x = r.origin().x() + t*r.direction().x();
        // auto y = r.origin().y() + t*r.direction().y();    

        // let x = ray.origin.a + t * ray.direction.a;
        // let y = ray.origin.b + t * ray.direction.b;
        // let z = ray.origin.c + t * ray.direction.c;

        // print!("Ray with t: {:?}\n", ray);

        // if x < self.p1.a || x > self.p2.a {
        //     return None;
        // }

        // if y < self.p1.b || y > self.p2.b {
        //     return None;
        // }

        // if z < self.p1.c || z > self.p2.c {
        //     return None;
        // }

        Some(RayCollision::new(ray, self.n, t, self.material))
    }
}