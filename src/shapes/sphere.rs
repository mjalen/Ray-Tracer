use crate::math::vector::*;
use crate::math::ray::Ray;
use crate::util::material::*;
use crate::util::hittable::*;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Material,
    t_min: f32,
    t_max: f32
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Material, t_min: f32, t_max: f32) -> Self {
        Sphere { center, radius, material, t_min, t_max }
    }

    pub fn new_pos_t(center: Point, material: Material, radius: f32) -> Self {
        // to account for floating point errors. ignore hits near 0.
        // this helps fix shadow acne... which is apparently a thing.
        let t_min: f32 = 0.001; 
        let t_max: f32 = f32::INFINITY;

        Sphere { center, radius, material, t_min, t_max }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray) -> Option<RayCollision> {
        // t^2*b*b + 2tb*(A-C)+(A-C)*(A-C) t solved using quadratic formula
        // optimized to t = -h +- sqrt(h^2 - ac) all over a
        let o_min_c: Point = ray.origin + self.center.scalar_mul(-1.0);

        let a: f32 = Vector3::dot(&ray.direction, &ray.direction);
        let half_b: f32 = Vector3::dot(&ray.direction, &o_min_c);
        let c: f32 = Vector3::dot(&o_min_c, &o_min_c) - self.radius * self.radius;

        let discriminant: f32 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // finish quad formula to get ray distance. 

        let t_root: f32 = (-1.0 * half_b - discriminant.sqrt()) / a;
        if t_root < self.t_min || t_root > self.t_max {
            return None
        }

        let norm: Point = (ray.at(t_root) + self.center.scalar_mul(-1.0)).scalar_div(self.radius); 
        
        Some(RayCollision::new(ray, norm, t_root, self.material.to_owned())) 
    }
}
