pub mod vector;
pub mod ray;
pub mod matrix;

use crate::math::vector::*;

// I like degrees, fight me.
pub fn deg_to_rad(degree: f32) -> f32 {
    use std::f32::consts::PI;
    let conversion: f32 = PI / 180.0; 

    let result: f32 = conversion * degree;

    println!("{} degrees = {} radians", degree, result);
    result
}

pub fn lerp_vec(start: Vector3, end: Vector3, t: f32) -> Vector3 {
    start.scalar_mul(1.0 - t) + end.scalar_mul(t) 
}

pub fn lerp(start: f32, end: f32, t: f32) -> f32 {
    (1.0 - t) * start + t * end
}
