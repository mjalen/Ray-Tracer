use crate::util::image::Image;

use crate::math;
use math::vector::Vector3 as Point;
use math::matrix::Mat3b3;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
    pub up: Point,
    pub to: Point,
    pub at: Point,
    pub ll_corner: Point,
    pub right: Point,
}

impl Camera {
    pub fn new(up: Point, at: Point, to: Point, image: Image) -> Self {
        // vector up is inferred to be pointing from bottom -> top of view. 
        let aspect_ratio: f32 = image.width as f32 / image.height as f32;
        let view_height: f32 = at.len();
        let view_width: f32 = view_height * aspect_ratio;

        // focal length is at.len()

        println!("View is {}x{}", view_width, view_height);
        println!("Focal length: {}", at.len());

        // there is a floating point errors in Mat3b3::roll... probably
        let right_dir = Mat3b3::roll(up, math::deg_to_rad(270.0));
        let right: Point = right_dir.scalar_mul(view_width);

        
        println!("Camera UP: {};\nCamera RIGHT: {}", up.to_string(), right.to_string());
        println!("Camera AT: {};\nCamera TO: {}", at.to_string(), to.to_string());

        let ll_corner: Point = {
            let unscaled_corner: Point = up.scalar_mul(-0.5) + right.scalar_mul(-0.5) + to;
            unscaled_corner + at.scalar_mul(-1.0)
        };

        println!("Camera LL Corner: {}", ll_corner.to_string());

        Camera { up, to, at, ll_corner, right}
    }
}
