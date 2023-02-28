pub mod util;
pub mod math;
use crate::util::image::{Image, RenderObject, DrawHeader};
use crate::math::vector::Vector3;
use crate::util::camera::Camera;
use crate::util::hittable::*;
use crate::util::material::*;

use std::time::Instant;

type Point = Vector3;
type Color = Vector3;

fn main() -> std::io::Result<()> {
    // calculate program run-time.
    let now = Instant::now();

    // image settings
    let aspect_ratio: f32 = 16.0 / 10.0;
    let width: i32 = 800;
    let height: i32 = (width as f32 / aspect_ratio) as i32;

    let image = Image::new(width, height);
    let camera: Camera = Camera::new(
        Point::new(0.0, 1.0, 0.0), // up
        Point::new(0.0, 0.0, 1.0), // at 
        Point::origin(),           // to
        image
    );

    // Materials
    let mat_ground = Material::Lambertian { albedo: Color::new(0.3, 0.8, 0.2) };
    let mat_center = Material::Lambertian { albedo: Color::new(0.8, 0.0, 0.0) };
    let mat_left = Material::Metal { albedo: Color::new(0.8, 0.8, 0.8) };
    let mat_right = Material::Metal { albedo: Color::new(0.8, 0.6, 0.2) };

    // all render shapes
    let world: World = World::new()
        .insert(Box::new(Sphere::new_pos_t(Point::new(0.0, 0.0, -1.0), mat_center, 0.35)))
        .insert(Box::new(Sphere::new_pos_t(Point::new(-0.7, 0.0, -1.0), mat_left, 0.3)))
        .insert(Box::new(Sphere::new_pos_t(Point::new(0.7, 0.0, -1.0), mat_right, 0.3)))

    // DONE Figure out why floor sphere is clipping above the camera.
    // FIX Make sure root t is positive.
        .insert(Box::new(Sphere::new_pos_t(Point::new(0.0,-100.5, -1.0), mat_ground, 100.0)));

    // render procedure 
    let render_closure = |render: RenderObject| -> Color {
        let camera: &Camera = render.camera;

        // sample coordinate from many random offsets
        let color: Color = camera.sample_pixel(render);
        color.scalar_mul(255.0)
    };

    // render
    let header: DrawHeader = DrawHeader {
        output_file: "output.ppm",
        camera: &camera,
        world: &world,
        draw_call: render_closure
    };

    image.draw(header)?; 

    // record how long program took.
    let execution_time = now.elapsed();
    println!("Execution Time: {} seconds.", execution_time.as_secs());

    Ok(())
}
