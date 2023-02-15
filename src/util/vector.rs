#[derive(Copy, Clone, Debug)]
pub struct Vector3 {
    pub a: f32,
    pub b: f32, 
    pub c: f32,
}

impl Vector3 {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Vector3 { a, b, c }
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

    pub fn dot(&self, other: &Self) -> f32 {
        self.a * other.a + self.b * other.b + self.c * other.c
    }

    pub fn cross(self, other: Vector3) -> Self {
        Self::new(
            self.b * other.c - self.c * other.b,
            self.c * other.a - self.a * other.c,
            self.a * other.b - self.b * other.a
        )
    }

    pub fn unit(self) -> Self {
        self.scalar_div(3.0)
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
