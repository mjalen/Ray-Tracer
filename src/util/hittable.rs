use crate::math::vector::Vector3;
use crate::math::ray::Ray;

type Point = Vector3;

// Trait for render-able objects in the world.
pub trait Hittable {
    fn hit(&self, ray: Ray) -> Option<RayCollision>;
}

#[derive(Copy, Clone, Debug)]
pub struct RayCollision { // returned when an object is hit by a ray.
    pub hit_point: Point,
    pub normal: Point,
    pub distance: f32,
    pub front_face: bool
}

impl RayCollision {
    pub fn new(ray: Ray, normal: Point, distance: f32) -> Self {

        // positive dot product -> vectors are in same direction.
        // negative dot product -> vectors are different directions.
        
        // the objective here is to have the normal vector
        // always oppose the casted ray while tracking
        // if the normal was originally outward facing.
        let is_outward: bool = ray.direction.dot(&normal) < 0.0; 
        let outward_normal: Point = if is_outward { normal } 
                                    else { normal.scalar_mul(-1.0) }; 

        RayCollision { 
            hit_point: ray.at(distance), 
            normal: outward_normal, 
            distance,
            front_face: is_outward
        }
    }
}

/*
 *  The World of Hittables 
 */

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



#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    t_min: f32,
    t_max: f32
}

impl Sphere {
    pub fn new(center: Point, radius: f32, t_min: f32, t_max: f32) -> Self {
        Sphere { center, radius, t_min, t_max }
    }

    pub fn new_pos_t(center: Point, radius: f32) -> Self {
        // to account for floating point errors. ignore hits near 0.
        // this helps fix shadow acne... which is apparently a thing.
        let t_min: f32 = 0.001; 
        let t_max: f32 = f32::INFINITY;

        Sphere { center, radius, t_min, t_max }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray) -> Option<RayCollision> {
        // t^2*b*b + 2tb*(A-C)+(A-C)*(A-C) t solved using quadratic formula
        // optimized to t = -h +- sqrt(h^2 - ac) all over a
        let o_min_c: Point = ray.origin + self.center.scalar_mul(-1.0);
        let a: f32 = ray.direction.dot(&ray.direction); 
        let half_b: f32 = ray.direction.dot(&o_min_c);
        let c: f32 = o_min_c.dot(&o_min_c) - self.radius * self.radius;

        let discriminant: f32 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // finish quad formula to get ray distance. 
        let mut t_root: f32 = (-1.0 * half_b - discriminant.sqrt()) / a;
        if t_root < self.t_min || t_root > self.t_max {
            t_root = (-1.0 * half_b - discriminant.sqrt()) / a;
            if t_root < self.t_min || t_root > self.t_max {
                return None;
            }
        }

        let norm: Point = (ray.at(t_root) + self.center.scalar_mul(-1.0)).scalar_div(self.radius); 
        
        Some(RayCollision::new(ray, norm, t_root)) 

    }
}


