use crate::math::*;
use crate::math::vector::*;
use crate::util::hittable::*;
use crate::util::material::*;

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
        // if diffusion depth has been reached 
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        } 

        let collision_result: Option<RayCollision> = world.hit(self);

        match collision_result {
            Some(c) => {
                // receive material dependent scatter ray
                let scatter = Material::scatter(self, &c);

                if !scatter.normal_matches {
                    return Color::new(0.0, 0.0, 0.0);
                }

                return scatter.attenuation * scatter.ray.ray_color(world, depth - 1); 
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
