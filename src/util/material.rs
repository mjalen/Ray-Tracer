use crate::math::ray::Ray;
use crate::util::hittable::RayCollision;
use crate::math::vector::Vector3;

type Color = Vector3;
type Point = Vector3;

#[derive(Copy, Clone, Debug)]
pub enum Material {
    Metal { albedo: Color },
    Lambertian { albedo: Color }
}

impl Material {
    pub fn scatter(self, r_in: Ray, collision: RayCollision) -> (Ray, Color, bool) {
        match self {
            Material::Metal { albedo } => Material::metal_scatter(albedo, r_in, collision),
            Material::Lambertian { albedo } => Material::lambertian_scatter(albedo, collision)
        }
    }

    fn lambertian_scatter(albedo: Color, collision: RayCollision) -> (Ray, Color, bool) {
        // standard unit length lambertian scatter with attenuated color
        let mut scatter_dir: Point = collision.normal + Vector3::rand_in_hemisphere(collision.normal);
       
        // in case scatter dir is the oppose of the normal.
        if scatter_dir.near_zero() {
            scatter_dir = collision.normal; 
        }

        let scattered = Ray::new(collision.hit_point, scatter_dir);
        let attenuation = albedo; 
        (scattered, attenuation, true)
    }

    // TODO Find out why metal material is showing as black sphere.
    fn metal_scatter(albedo: Color, r_in: Ray, collision: RayCollision) -> (Ray, Color, bool) {
        let mut reflection: Point = r_in.direction.unit().reflect(collision.normal);
        let norm_matches_reflection: bool = reflection.dot(collision.normal) > 0.0;

        if !norm_matches_reflection {
            // if this is not done, the reflection is upside-down and is a blob.
            reflection = reflection.scalar_mul(-1.0);
        }

        let scattered: Ray = Ray::new(collision.hit_point, reflection);
        
        (scattered, albedo, true)
    }

}
