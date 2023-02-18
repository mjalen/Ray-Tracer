use crate::util::image::Image;

use crate::math::*;
use crate::math::vector::Vector3;
use crate::math::matrix::Mat3b3;
use crate::math::ray::Ray;
use crate::util::image::*;

type Point = Vector3;
type Color = Vector3;

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
        let right_dir = Mat3b3::roll(up, deg_to_rad(270.0));
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

    pub fn get_ray(self, u: f32, v: f32) -> Ray {
        let dir: Point = self.ll_corner + self.right.scalar_mul(u) + self.up.scalar_mul(v);

        Ray::new(Point::origin(), dir)
    }

    // returns the average color found by sampling around a pixel
    pub fn sample_pixel(self, context: RenderObject) -> Color {
        let mut pixel: Color = Color::origin();
        for _ in 0..context.image.samples_per_pixel {
            // closure for offseting the sample ray.
            let sample_offset = |original: f32, length: f32| -> f32 {
                let numerator: f32 = original + random_f32(0.0, 1.0);
                numerator / (length - 1.0)
            };

            let u: f32 = sample_offset(context.coordinate.a, context.image.width as f32);
            let v: f32 = sample_offset(context.coordinate.b, context.image.height as f32);

            let r: Ray = self.get_ray(u, v);
            pixel = pixel + r.ray_color(context.world, 50);
        }

        let scale: f32 = 1.0 / context.image.samples_per_pixel as f32;
        let color: Color = pixel.scalar_mul(scale);

        // sqrt is gamma 2 color correction:
        // color ^ (1/gamma)
        Color::new(
            clamp(color.a.sqrt(), 0.0, 0.999),
            clamp(color.b.sqrt(), 0.0, 0.999),
            clamp(color.c.sqrt(), 0.0, 0.999)
        )
    }
}
