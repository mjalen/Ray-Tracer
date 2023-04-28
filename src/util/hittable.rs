// use std::thread;

use crate::math::vector::Vector3;
use crate::math::ray::Ray;
use crate::util::material::*;

use crate::shapes::shape::*;

type Point = Vector3;

// Trait for render-able objects in the world.
pub trait Hittable {
    fn hit(&self, ray: Ray) -> Option<RayCollision>;
}

pub trait Collision {}

#[derive(Copy, Clone, Debug)]
pub struct RayCollision { // returned when an object is hit by a ray.
    pub hit_point: Point, // actual point of collision.
    pub normal: Point, // collided surface's normal from hit_point 
    pub distance: f32, // distance from camera to collision.
    pub front_face: bool, // did the ray collide the inside or outside (front) of the surface?
    pub material: Material // the type of material collided
}

impl RayCollision {
    pub fn new(ray: Ray, normal: Point, distance: f32, material: Material) -> Self {

        // positive dot product -> vectors are in same direction.
        // negative dot product -> vectors are different directions.
        
        // the objective here is to have the normal vector
        // always oppose the casted ray while tracking
        // if the normal was originally outward facing.
        let is_outward: bool = ray.direction.dot(normal) < 0.0; 
        let outward_normal: Point = if is_outward { normal } 
                                    else { normal.scalar_mul(-1.0) }; 

        RayCollision { 
            hit_point: ray.at(distance), 
            normal: outward_normal, 
            distance,
            front_face: is_outward,
            material 
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

    pub fn insert(mut self, object: Shape) -> Self {
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
        // calculate each collision through ray. 
        let hits = self.objects 
            .iter()
            .map(|obj| obj.hit(ray))
            .collect::<Vec<Option<RayCollision>>>();

        // find closest hit to camera
        let mut minimum: Option<RayCollision> = None;
        for collision in hits {
            if let Some(c) = collision {
                match minimum {
                    Some(min) => {
                        if min.distance > c.distance {
                            minimum = Some(c)
                        }
                    },
                    None => minimum = Some(c)
                };   
            }
        }

        minimum
    }
}