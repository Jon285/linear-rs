use std::ops::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[allow(dead_code)]
impl Vec4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    #[inline]
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    #[inline]
    pub fn dot(&self, other: &Vec4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    #[inline]
    pub fn normalize(&mut self) {
        let s = 1.0 / self.len();

        self.x *= s;
        self.y *= s;
        self.z *= s;
        self.w *= s;
    }
}

impl_vec_ops!(Vec4, x, y, z, w);
