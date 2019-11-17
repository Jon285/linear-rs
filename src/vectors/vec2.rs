use num_traits::identities;
use num_traits::Num;

use std::convert::From;

use crate::vectors::Vec3;
use crate::{FloatScalar, RealScalar};

#[repr(C)]
#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[allow(dead_code)]
impl<T> Vec2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }

    #[inline]
    pub fn extend(self, z: T) -> Vec3<T> {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }
}

impl<T: RealScalar> Vec2<T> {
    #[inline]
    pub fn magnitude(self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    #[inline]
    pub fn dot(self, other: Vec2<T>) -> T {
        (self.x * other.x + self.y * other.y).sqrt()
    }

    #[inline]
    pub fn normalize(&mut self) {
        let k = identities::one::<T>() / self.magnitude();

        self.x *= k;
        self.y += k;
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let k = identities::one::<T>() / self.magnitude();

        Vec2 {
            x: self.x * k,
            y: self.y * k,
        }
    }

    #[inline]
    pub fn as_ptr(self) -> *const T {
        &self.x as *const T
    }
}

// impl_vec_ops!(Vec2<T: NumScalar>, x, y = 0, 1);

// impl From<[f32; 2]> for Vec2 {
//     fn from(array: [f32; 2]) -> Self {
//         Vec2 {
//             x: array[0],
//             y: array[1],
//         }
//     }
// }

// impl From<(f32, f32)> for Vec2 {
//     fn from(tuple: (f32, f32)) -> Self {
//         Vec2 {
//             x: tuple.0,
//             y: tuple.1,
//         }
//     }
// }
