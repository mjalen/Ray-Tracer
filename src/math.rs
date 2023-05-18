pub mod vector;
pub mod ray;

use crate::math::vector::*;

use rand::prelude::*;


// I like degrees, fight me.
pub fn deg_to_rad(degree: f32) -> f32 {
    use std::f32::consts::PI;
    let conversion: f32 = PI / 180.0; 

    let result: f32 = conversion * degree;

    println!("{} degrees = {} radians", degree, result);
    result
}

pub fn random_f32(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let y: f32 = rng.gen();

    (max - min) * y + min 
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min; }
    if x > max { return max; }
    x
}

pub fn lerp_vec(start: Vector3<f32>, end: Vector3<f32>, t: f32) -> Vector3<f32> {
    start.scalar_mul(1.0 - t) + end.scalar_mul(t) 
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    (1.0 - t) * start + t * end
}
