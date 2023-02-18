use crate::math::*;
use crate::math::vector::Vector3;
use crate::util::hittable::*;

type Point = Vector3;
type Color = Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Point
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(self, t: f32) -> Point {
        self.origin + self.direction.scalar_mul(t)
    }

    pub fn ray_color(self, world: &World, depth: i32) -> Color {
        match world.hit(self) {
            Some(c) => {
                // if diffusion depth has been reached 
                if depth <= 0 {
                    return Color::new(0.0, 0.0, 0.0);
                }

                let target: Point = c.hit_point + c.normal + Vector3::rand_in_hemisphere(&c.normal);
                let diffuse_ray: Ray = Ray::new(c.hit_point, target + c.hit_point.scalar_mul(-1.0));

                diffuse_ray.ray_color(world, depth - 1).scalar_mul(0.5)
                // Color::new(c.normal.a + 1.0, c.normal.b + 1.0, c.normal.c + 1.0).scalar_mul(0.5)
            },
            None => {
                // background color
                let t: f32 = 0.5 * ( self.direction.b + 1.0 ); 

                let white: Color = Color::new(1.0, 1.0, 1.0);
                let blue: Color = Color::new(0.5, 0.7, 1.0);

                lerp_vec(white, blue, t)
            }
        }
    }
}
