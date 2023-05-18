// use std::thread;

use std::f32::consts::PI;

use crate::math::vector::*;
use crate::math::ray::Ray;
use crate::util::material::*;

use crate::shapes::shape::*;

// Trait for render-able objects in the world.
pub trait Hittable {
    fn hit(&self, ray: Ray) -> Option<RayCollision>;
}

pub trait Collision {}

#[derive(Clone, Debug)]
pub struct RayCollision { // returned when an object is hit by a ray.
    pub hit_point: Point, // actual point of collision.
    pub normal: Point, // collided surface's normal from hit_point 
    pub distance: f32, // distance from camera to collision.
    pub front_face: bool, // did the ray collide the inside or outside (front) of the surface?
    pub uv: Point,
    pub material: Material // the type of material collided
}

impl RayCollision {
    pub fn new(ray: Ray, normal: Point, distance: f32, material: Material) -> Self {

        // positive dot product -> vectors are in same direction.
        // negative dot product -> vectors are different directions.
        
        // the objective here is to have the normal vector
        // always oppose the casted ray while tracking
        // if the normal was originally outward facing.
        let is_outward: bool = Vector3::dot(&ray.direction, &normal) < 0.0;
        let outward_normal: Point = if is_outward { normal } 
                                    else { normal.scalar_mul(-1.0) }; 

        // let hit_point = ray.at(distance);
        let u = normal.c.atan2(-1.0 * outward_normal.a) / (2.0 * PI);
        let v = (-1.0 * outward_normal.b).acos() / PI;

        RayCollision { 
            hit_point: ray.at(distance), 
            normal: outward_normal, 
            distance,
            front_face: is_outward,
            material,
            uv: Point::new(u, v, 0.0)
        }
    }
}

/*
 *  The World of Hittables 
 */

#[derive(Clone)]
pub struct World {
    pub objects: Vec<Shape>,
}

impl World {
    pub fn new() -> Self {
        World { objects: vec![] }
    }

    pub fn insert<'a>(&mut self, object: Shape) {
        self.objects.push(object);
    }

    pub fn clear(mut self) -> Self {
        self.objects = vec![];
        self
    }
}

impl Hittable for World {
    fn hit(&self, ray: Ray) -> Option<RayCollision> {
        // calculate each collision through ray. 
        // my god this is a nightmare O.O
        self.objects 
            .iter()
            .map(|obj| obj.hit(ray))
            .filter(|obj| {
                match obj {
                    Some(_) => return true,
                    None => return false
                }
            })
            .map(|obj| obj.unwrap())
            .min_by(|x, y| {
                x.distance.to_owned().total_cmp(&y.distance)
            })
    }
}