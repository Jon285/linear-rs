use num_traits::Num;

use std::convert::From;

use crate::vectors::Vec3;

pub trait Scalar: Num + Copy + Clone + PartialEq + Default {}

#[repr(C)]
#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Vec2<T: Scalar> {
    pub x: T,
    pub y: T,
}

#[allow(dead_code)]
impl<T: Num> Vec2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    #[inline]
    pub fn magnitude(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    #[inline]
    pub fn dot(self, other: Vec2) -> f32 {
        (self.x * other.x + self.y * other.y).sqrt()
    }

    #[inline]
    pub fn normalize(&mut self) {
        let k = 1.0 / self.magnitude();

        self.x *= k;
        self.y += k;
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let k = 1.0 / self.magnitude();

        Vec2 {
            x: self.x * k,
            y: self.y * k,
        }
    }

    #[inline]
    pub fn extend(self, z: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }

    #[inline]
    pub fn as_ptr(self) -> *const f32 {
        &self.x as *const f32
    }
}

impl_vec_ops!(Vec2, x, y = 0, 1);

impl From<[f32; 2]> for Vec2 {
    fn from(array: [f32; 2]) -> Self {
        Vec2 {
            x: array[0],
            y: array[1],
        }
    }
}

impl From<(f32, f32)> for Vec2 {
    fn from(tuple: (f32, f32)) -> Self {
        Vec2 {
            x: tuple.0,
            y: tuple.1,
        }
    }
}
