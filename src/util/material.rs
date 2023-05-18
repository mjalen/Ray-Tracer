use crate::math::ray::Ray;
use crate::util::hittable::RayCollision;
use crate::math::vector::*;

use super::texture::Texture;

#[derive(Clone, Debug)]
pub enum Material {
    Metal(Color),
    Lambertian(Texture),
}

pub struct ScatterResult {
    pub ray: Ray,
    pub attenuation: Color,
    pub normal_matches: bool,
}

impl Material {
    pub fn scatter(r_in: Ray, collision: &RayCollision) -> ScatterResult {
        match &collision.material {
            Material::Metal(albedo ) => Material::metal_scatter(albedo.to_owned(), r_in, collision),
            Material::Lambertian(texture) => Material::lambertian_scatter(texture, collision),
        }
    }

    fn lambertian_scatter(texture: &Texture, collision: &RayCollision) -> ScatterResult {
        // standard unit length lambertian scatter with attenuated color
        let mut scatter_dir: Point = collision.normal + Vector3::rand_in_unit_sphere();
       
        // in case scatter dir is the oppose of the normal.
        if scatter_dir.near_zero() {
            scatter_dir = collision.normal; 
        }

        let scattered = Ray::new(collision.hit_point, scatter_dir);

        let attenuation = texture.get(&collision.uv.a, &collision.uv.b); 

        ScatterResult { 
            ray: scattered, 
            attenuation, 
            normal_matches: true, 
        }
    }

    fn metal_scatter(albedo: Color, r_in: Ray, collision: &RayCollision) -> ScatterResult {
        let mut reflection: Point = r_in.direction.unit().reflect(collision.normal);
        let norm_matches_reflection: bool = Vector3::dot(&reflection, &collision.normal) > 0.0;

        if !norm_matches_reflection {
            // if this is not done, the reflection is upside-down and is a blob.
            reflection = reflection.scalar_mul(-1.0);
        }

        let scattered: Ray = Ray::new(collision.hit_point, reflection);
        
        ScatterResult {
            ray: scattered,
            attenuation: albedo,
            normal_matches: true,
        }
    }
}
