pub mod util;
pub mod math;
use crate::util::image::{Image, RenderObject, DrawHeader};
use crate::math::vector::Vector3;
use crate::math::ray::Ray;
use crate::util::camera::Camera;
use crate::util::hittable::*;
use crate::math::*;

type Point = Vector3;
type Color = Vector3;

fn main() -> std::io::Result<()> {
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

    // all render shapes
    let world: World = World::new()
        .insert(Box::new(Sphere::new(Point::new(-0.25, -0.25, -1.0), 0.15)))
        .insert(Box::new(Sphere::new(Point::new(0.25, -0.25, -1.0), 0.15)))
        .insert(Box::new(Sphere::new(Point::new(0.0, 0.25, -1.0), 0.15)));

    // TODO Figure out why floor sphere is clipping above the camera.
    //  .insert(Box::new(Sphere::new(Point::new(0.0,-100.5,-1.0), 100.0)));

    // render procedure 
    let render_closure = |render: RenderObject| -> Color {
        let camera: &Camera = render.camera;

        let r: Ray = Ray::new(
            Point::origin(),
            camera.ll_corner + camera.right.scalar_mul(render.coordinate.a) 
            + camera.up.scalar_mul(render.coordinate.b) 
            + camera.to.scalar_mul(-1.0) 
        );
        
        let color: Color = ray_color(r, render.world).scalar_mul(255.0);
        color
    };

    // render
    let header: DrawHeader = DrawHeader {
        output_file: "output.ppm",
        camera: &camera,
        world: &world,
        draw_call: render_closure
    };

    image.draw(header)?; 
    Ok(())
}

fn ray_color(r: Ray, world: &World) -> Color {
    match world.hit(r) {
        Some(c) => {
            // sphere colorized to normals
            Color::new(c.normal.a + 1.0, c.normal.b + 1.0, c.normal.c + 1.0).scalar_mul(0.5)
        },
        None => {
            // background color
            let t: f32 = 0.5 * ( r.direction.b + 1.0 ); 

            let white: Color = Color::new(1.0, 1.0, 1.0);
            let blue: Color = Color::new(0.5, 0.7, 1.0);

            lerp_vec(white, blue, t)
        }
    }
}
