use crate::math::random_f32;
use crate::math::matrix::Mat3b3;

#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub a: f32, // given a, b, c to be Point and Color independent.
    pub b: f32, 
    pub c: f32,
}

pub type Point = Vector3;
pub type Color = Vector3;

impl Vector3 {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Vector3 { a, b, c }
    }

    pub fn random(min: f32, max: f32) -> Self {
        Vector3 { a: random_f32(min, max), b: random_f32(min, max), c: random_f32(min, max) }
    }

    pub fn rand_in_unit_sphere() -> Point {
        // random length scattering 
        // for Lambertian Distribution.
        loop {
            let s: Point = Vector3::random(-1.0, 1.0);
            if s.len() * s.len() < 1.0 {
                return s;
            }
        }
    }

    pub fn rand_unit_vec() -> Point {
        // unit length scattering 
        // Used for a more proper Lambertian Distribution of light ray diffusion (Incorrect)
        Point::rand_in_unit_sphere()
    }

    pub fn rand_in_hemisphere(normal: Point) -> Point {
        // hemispherical scattering
        // diffusion in same hemisphere as normal
        let in_unit_sphere: Point = Vector3::rand_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        }
        in_unit_sphere.scalar_mul(-1.0)
    }

    pub fn near_zero(self) -> bool {
        // to avoid normal - scatter vector = 0
        let s = 1e-8;

        // return if vector is near origin
        self.a.abs() < s &&  
        self.b.abs() < s &&
        self.c.abs() < s
    }

    pub fn reflect(self, normal: Vector3) -> Self {
        // simulate metal reflection:
        // reflection = v + 2*b.
        // v is self.
        // b is the normal vector of length v dot n
        //
        // v: (0.3, 0.4, 0.5)
        // n: (0.5, 0.5, 0.5)
        
        let b: Point = normal.scalar_mul(self.dot(normal));
        let refl: Point = self + b.scalar_mul(2.0);
        refl
    }

    pub fn origin() -> Self {
        Vector3::new(0.0, 0.0, 0.0)
    }

    pub fn to_string(self) -> String {
        format!("({}, {}, {})", self.a, self.b, self.c)
    }

    pub fn to_pixel(self) -> String {
        format!("{} {} {}\n", self.a as i32, self.b as i32, self.c as i32)
    }

    pub fn scalar_mul(self, factor: f32) -> Self {
        Self::new(self.a * factor, self.b * factor, self.c * factor) 
    }

    pub fn scalar_div(self, denominator: f32) -> Self {
        self.scalar_mul(1.0 / denominator)
    }

    pub fn dot(self, other: Self) -> f32 {
        self.a * other.a + self.b * other.b + self.c * other.c
    }

    pub fn cross(self, other: Vector3) -> Self {
        Self::new(
            self.b * other.c - self.c * other.b,
            self.c * other.a - self.a * other.c,
            self.a * other.b - self.b * other.a
        )
    }

    pub fn mul_m(self, mat: Mat3b3) -> Self {
        let a_total: f32 = self.a * mat.c1.a + self.b * mat.c2.a + self.c * mat.c3.a;
        let b_total: f32 = self.a * mat.c1.b + self.b * mat.c2.b + self.c * mat.c3.b;
        let c_total: f32 = self.a * mat.c1.c + self.b * mat.c2.c + self.c * mat.c3.c;

        Self::new(a_total, b_total, c_total)
    }

    pub fn unit(self) -> Self {
        self.scalar_div(self.len())
    }

    pub fn len(self) -> f32 {
        f32::sqrt(self.a * self.a + self.b * self.b + self.c * self.c) 
    }
}

use std::ops::Add;
impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.a + other.a,
            self.b + other.b,
            self.c + other.c
        )
    }
}


use std::ops::Mul;
impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.a * rhs.a,
            self.b * rhs.b,
            self.c * rhs.c
        )
    }
}
