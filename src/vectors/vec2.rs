use std::ops::*;

#[repr(C)]
#[derive(Debug)]
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
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
