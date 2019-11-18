use num_traits::identities;

use std::convert::From;

use crate::vectors::Vec3;
use crate::FloatScalar;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[allow(dead_code)]
impl<T> Vec4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Vec4 { x, y, z, w }
    }

    #[inline]
    pub fn as_ptr(self) -> *const T {
        &self.x as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        &mut self.x as *mut T
    }

    #[inline]
    pub fn truncate(self) -> Vec3<T> {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<T: FloatScalar> Vec4<T> {
    #[inline]
    pub fn magnitude(self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    #[inline]
    pub fn squared_mag(self) -> T {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)
    }

    #[inline]
    pub fn dot(self, other: Vec4<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let s = identities::one::<T>() / self.magnitude();

        Vec4 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,
        }
    }

    #[inline]
    pub fn normalize(&mut self) {
        let s = identities::one::<T>() / self.magnitude();

        self.x *= s;
        self.y *= s;
        self.z *= s;
        self.w *= s;
    }
}

impl_vec_ops!(Vec4, x, y, z, w = 0, 1, 2, 3);

impl<T> From<[T; 4]> for Vec4<T> {
    fn from(array: [T; 4]) -> Self {
        Vec4 {
            x: array[0],
            y: array[1],
            z: array[2],
            w: array[3],
        }
    }
}

impl<T> From<(T, T, T, T)> for Vec4<T> {
    fn from(tuple: (T, T, T, T)) -> Self {
        Vec4 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
            w: tuple.3,
        }
    }
}
