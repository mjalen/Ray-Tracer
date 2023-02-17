use crate::math::vector::Vector3;
use crate::math::ray::Ray;

type Point = Vector3;

#[derive(Copy, Clone, Debug)]
pub struct RayCollision { // returned when an object is hit by a ray.
    pub hit_point: Point,
    pub normal: Point,
    pub distance: f32,
    pub front_face: bool
}

impl RayCollision {
    pub fn new(ray: Ray, normal: Point, distance: f32) -> Self {
        // if normal points into shape, reverse it.
        let is_inward: bool = ray.direction.dot(&normal) < 0.0; 
        let outward_normal: Point = if is_inward { normal } else { normal.scalar_mul(-1.0) }; 

        RayCollision { 
            hit_point: ray.at(distance), 
            normal: outward_normal, 
            distance,
            front_face: is_inward
        }
    }
}

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        World { objects: vec![] }
    }

    pub fn insert(mut self, object: Box<dyn Hittable>) -> Self {
        self.objects.push(object);
        self
    }

    pub fn clear(mut self) -> Self {
        self.objects = vec![];
        self
    }
}

impl Hittable for World {
    fn hit(&self, ray: Ray) -> Option<RayCollision> {
        let hits: Vec<Option<RayCollision>> = self.objects
            .iter()
            .map(|obj| obj.hit(ray))
            .collect();

        let mut minimum: Option<RayCollision> = None;
        for collision in hits {
            let _ = match collision {
                Some(c) => {
                    match minimum {
                        Some(min) => if min.distance > c.distance { minimum = Some(c) },
                        None => minimum = Some(c)
                    }
                },
                None => {}
            };
        }

        minimum
    }
}

// trait for all render objects
pub trait Hittable {
    fn hit(&self, ray: Ray) -> Option<RayCollision>;
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
    fn hit(&self, ray: Ray) -> Option<RayCollision> {
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
        let norm: Point = (ray.at(dist) + self.center.scalar_mul(-1.0)).unit(); 
        
        Some(RayCollision::new(ray, norm, dist)) 

    }
}


