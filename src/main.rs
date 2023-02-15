pub mod util;
use crate::util::image::Image;
use crate::util::vector::Vector3;
use crate::util::ray::Ray;

type Point = Vector3;
type Color = Vector3;

fn main() -> std::io::Result<()> {
    // image settings
    let aspect_ratio: f32 = 16.0 / 10.0;
    let width: i32 = 800;
    let height: i32 = (width as f32 / aspect_ratio) as i32;
    let image = Image::new(width, height);

    // render procedure 
    let render_closure = |position: Point| -> Color {
        let viewport_height = 1.0;
        let viewport_width = 16.0 * viewport_height / 10.0;
        let focal_length = 1.0;
        let horizontal: Point = Point::new(viewport_width, 0.0, 0.0);
        let vertical: Point = Point::new(0.0, viewport_height, 0.0);
        let camera_center: Point = Point::new(0.0, 0.0, focal_length);

        let origin: Point = Point::new(0.0, 0.0, 0.0);
        let ll_corner: Point = origin + horizontal.scalar_mul(-0.5) + vertical.scalar_mul(-0.5) + camera_center.scalar_mul(-1.0);  

        let r: Ray = Ray::new(
            Point::new(0.0,0.0,0.0), 
            ll_corner + horizontal.scalar_mul(position.a) 
            + vertical.scalar_mul(position.b) 
            + origin.scalar_mul(-1.0) 
        );
        
        let color: Color = ray_color(r).scalar_mul(255.0);
        color
    };

    // render
    image.draw("output.ppm", render_closure)?; 
    Ok(())
}

fn is_circle_hit(center: Point, ray: Ray, radius: f32) -> bool {
    // t^2*b*b + 2tb*(A-C)+(A-C)*(A-C) t solved using quadratic formula
    let o_min_c: Point = ray.origin + center.scalar_mul(-1.0);
    let a: f32 = ray.direction.dot(&ray.direction); 
    let b: f32 = 2.0 * ray.direction.dot(&o_min_c);
    let c: f32 = o_min_c.dot(&o_min_c) - radius * radius;
    let formula_sqrt: f32 = b * b - 4.0 * a * c;
    println!("{}", formula_sqrt);
    formula_sqrt > 0.0
}

fn ray_color(r: Ray) -> Color {
    let center: Point = Point::new(0.0, 0.0, -1.0);
    if is_circle_hit(center, r, 0.25) {
        println!("!!!");
        return Color::new(0.0, 0.0, 1.0);
    }

    let t: f32 = 0.5 * ( r.direction.b + 1.0 ); 

    let white: Color = Color::new(1.0, 1.0, 1.0);
    let blue: Color = Color::new(0.5, 0.7, 1.0);

    lerp_vec(white, blue, t)
}

fn lerp_vec(start: Vector3, end: Vector3, t: f32) -> Vector3 {
    start.scalar_mul(1.0 - t) + end.scalar_mul(t) 
}

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    (1.0 - t) * start + t * end
}
