pub mod util;
pub mod math;
pub mod shapes;

use crate::shapes::plane::Plane;
use crate::util::image::{Image, DrawHeader};
use crate::math::vector::Vector3;
use crate::util::camera::Camera;
use crate::util::hittable::*;
use crate::util::material::*;
use crate::shapes::sphere::*;
use crate::shapes::shape::*;

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
    let fov: f32 = 90.0;

    let samples: i32 = 10; 

    let image = Image::new(width, height, fov, samples);
    let camera: Camera = Camera::new(
        Point::new(0.0, 1.0, 0.0), // up
        Point::new(0.0, 0.0, 0.1), // at 
        Point::new(0.0, 0.0, 0.0),           // to
        image
    );

    // Materials
    let mat_ground = Material::Lambertian { albedo: Color::new(0.3, 0.8, 0.2) };
    let mat_center = Material::Lambertian { albedo: Color::new(0.0, 0.2, 0.8) };
    let mat_left = Material::Metal { albedo: Color::new(0.8, 0.8, 0.8) };
    let mat_right = Material::Metal { albedo: Color::new(0.8, 0.6, 0.2) };

    // all render shapes
    let world: World = World::new()
        .insert(Shape::sphere(Sphere::new_pos_t(Point::new(0.0, -0.5, 0.0), mat_center, 0.2)))
        .insert(Shape::sphere(Sphere::new_pos_t(Point::new(0.0, 0.0, -1.0), mat_left, 0.45)))
        .insert(Shape::sphere(Sphere::new_pos_t(Point::new(-0.2, 0.0, 1.0), mat_left, 0.45)))
        // .insert(Shape::sphere(Sphere::new_pos_t(Point::new(-0.6, 0.0, -1.0), mat_left, 0.7)))
        // .insert(Shape::sphere(Sphere::new_pos_t(Point::new(0.6, 0.0, -1.0), mat_right, 0.7)))

    // DONE Figure out why floor sphere is clipping above the camera.
    // FIX Make sure root t is positive.
        .insert(Shape::sphere(Sphere::new_pos_t(Point::new(0.0,-101.0, -1.0), mat_left, 100.0)));

        // TODO Tether planes to axes, or figure out how to angle without a width
        // .insert(Shape::plane(Plane::new(
        //     Point::new(-0.1, -1.0, -0.1),
        //     Point::new(0.1, -1.01, 0.1),
        //     Point::new(0.0, 1.0, 0.0),
        //     mat_left)));

    // render
    let header: DrawHeader = DrawHeader {
        output_file: "output.ppm",
        camera: &camera,
        world: &world,
    };

    image.draw(&header)?; 

    // record how long program took.
    let execution_time = now.elapsed();
    println!("Execution Time: {} seconds.", execution_time.as_secs());

    Ok(())
}
