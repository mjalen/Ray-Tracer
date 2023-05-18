use crate::util::material::Material;
use crate::{math::vector::*, util::hittable::RayCollision};
use crate::math::ray::Ray;
use crate::util::hittable::*;

#[derive(Clone, Debug)]
pub struct Plane {
    pub min: Point,
    pub max: Point,
    pub n: Point,
    pub material: Material,
    pub t_min: f32,
    pub t_max: f32
}

impl Plane {
    pub fn new (min: Point, max: Point, n: Point, material: Material) -> Plane {
        let t_max = 0.001;
        let t_min = f32::INFINITY;

        Plane { min, max, n, material, t_min, t_max }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: Ray) -> Option<RayCollision> {

        let t = {
            let t_num = Vector3::dot(&self.min, &self.n) - Vector3::dot(&ray.origin, &self.n);
            let t_den = Vector3::dot(&ray.direction, &self.n);

            t_num / t_den
        };  

        if t < self.t_min || t > self.t_max {
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

        Some(RayCollision::new(ray, self.n, t, self.material.to_owned()))
    }
}