pub mod util;
pub mod math;
pub mod shapes;

use crate::util::image::{Image, DrawHeader};
use crate::math::vector::*;
use crate::util::camera::Camera;
use crate::util::hittable::*;
use crate::util::material::*;
use crate::shapes::sphere::*;
use crate::shapes::shape::*;
use crate::util::texture::Texture;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    // calculate program run-time.
    let now = Instant::now();

    // image settings
    let aspect_ratio: f32 = 16.0 / 10.0;
    let width: i32 = 1920;
    let height: i32 = (width as f32 / aspect_ratio) as i32;
    let fov: f32 = 80.0;

    let samples: i32 = 10; 

    let image = Image::new(width, height, fov, samples);
    let camera: Camera = Camera::new(
        Point::new(0.0, 1.0, 0.0), // up
        Point::new(0.0, 0.0, 0.1), // at 
        Point::new(0.0, 0.0, 0.0), // to
        image
    );

    // Materials
    // let mat_ground = Material::Lambertian(
    //     Texture::Solid(Color::new(0.3, 0.8, 0.2))
    // );

    let red = Material::Lambertian(
        Texture::Solid(Color::new(1.0, 0.0, 0.0))
    );

    // let checker = Material::Lambertian(
    //     Texture::from_img("assets/earthmap.jpg".to_owned())
    // );

    // 2 of the same material, because shapes take ownership of materials. 
    let metal_clear = Material::Metal(Color::new(0.8, 0.8, 0.8));
    let metal_clear2 = Material::Metal(Color::new(0.8, 0.8, 0.8));
    let metal_clear3 = Material::Metal(Color::new(0.8, 0.8, 0.8));

    // all render shapes
    let mut world = World::new();

    // For earth uv render.
    // world.insert(Shape::sphere(Sphere::new_pos_t(Point::new(0.0, 0.0, -0.1), checker, 0.4)));
    // world.insert(Shape::sphere(Sphere::new_pos_t(Point::new(-0.2, 0.0, 1.0), metal_clear, 0.45)));
    // world.insert(
    //     Shape::Sphere(
    //         Sphere::new_pos_t(Point::new(-0.2, 0.0, 1.0), metal_clear, 0.45)
    //     )
    // );

    // metal sphere behind the camera
    world.insert(
        Shape::sphere(
            Sphere::new_pos_t(
                Point::new(-0.2, 0.0, 1.0), 
                metal_clear, 
                0.4)));

    // matte sphere
    world.insert(
        Shape::Sphere(
            Sphere::new_pos_t(Point::new(0.0, -0.5, 0.0), red, 0.2)   
        )
    );

    // metal sphere in front of camera.
    world.insert(
        Shape::Sphere(
            Sphere::new_pos_t(Point::new(0.0, 0.0, -1.0), metal_clear2, 0.45)
        )
    );

    

    // GROUND 
    world.insert(
        Shape::sphere(
            Sphere::new_pos_t(
                Point::new(0.0,-101.0, -1.0), metal_clear3, 
                100.0)));

        // TODO Align planes to axes, or figure out how to angle a plane 

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
