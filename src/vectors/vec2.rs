use std::ops::*;

#[repr(C)]
#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vec2 {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x: x, y: y }
    }

    #[inline]
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    #[inline]
    pub fn dot(&self, other: &Vec2) -> f32 {
        (self.x * other.x + self.y * other.y).sqrt()
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let k = 1.0 / self.len();

        Vec2 {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

impl_vec_ops!(Vec2, x, y = 0, 1);
