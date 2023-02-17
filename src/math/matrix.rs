use crate::math::vector::Vector3;

type Point = Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Mat3b3 {
    pub c1: Vector3,
    pub c2: Vector3,
    pub c3: Vector3
}

impl Mat3b3 {
    pub fn new(c1: Vector3, c2: Vector3, c3: Vector3) -> Self {
        Mat3b3 { c1, c2, c3 }
    }

    pub fn pitch(radian: f32) -> Self { // along a-axis

        // returns:
        // 1   0     0 
        // 0   cos   -sin 
        // 0   sin  cos
        todo!()

    }

    pub fn yaw(radian: f32) -> Self { // along b-axis
        
        // returns:
        // cos   0    sin
        // 0     1    0
        // -sin   0    cos
        todo!()
    }

    pub fn roll(pos: Point, by_radian: f32) -> Point { // along c-axis
        let cos_rad: f32 = by_radian.cos();
        let sin_rad: f32 = by_radian.sin();

        let c1: Vector3 = Vector3::new(cos_rad, sin_rad, 0.0);
        let c2: Vector3 = Vector3::new(-1.0 * sin_rad, cos_rad, 0.0);
        let c3: Vector3 = Vector3::new(0.0, 0.0, 1.0);

        // rotation:
        // cos  sin  0
        // -sin  cos  0
        // 0    0    1
        let mat: Mat3b3 = Mat3b3::new(c1, c2, c3);

        pos.mul_m(mat)
    }

    
}
