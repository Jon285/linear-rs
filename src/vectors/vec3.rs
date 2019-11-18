use num_traits::identities;

use std::convert::From;

use crate::vectors::Vec2;
use crate::vectors::Vec4;

use crate::FloatScalar;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[allow(dead_code)]
impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }
}

impl<T: FloatScalar> Vec3<T> {
    #[inline]
    pub fn magnitude(self) -> T {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    #[inline]
    pub fn squared_mag(self) -> T {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    #[inline]
    pub fn normalize(&mut self) {
        let k = 1.0 / self.magnitude();
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let k = identities::one::<T>() / self.magnitude();
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    #[inline]
    pub fn dot(self, rhs: Vec3<T>) -> T {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    #[inline]
    pub fn cross(self, b: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: -(self.x * b.z - self.z * b.x),
            z: self.x * b.y - self.y * b.x,
        }
    }

    #[inline]
    pub fn distance_to(self, other: Vec3<T>) -> T {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2))
            .sqrt()
    }

    #[inline]
    pub fn vector_to(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    #[inline]
    pub fn extend(self, w: T) -> Vec4<T> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }

    #[inline]
    pub fn truncate(self) -> Vec2<T> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    pub fn as_ptr(self) -> *const T {
        &self.x as *const T
    }
}

impl_vec_ops!(Vec3, x, y, z = 0, 1, 2);

impl<T> From<[T; 3]> for Vec3<T> {
    fn from(array: [T; 3]) -> Self {
        Vec3 {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(tuple: (T, T, T)) -> Self {
        Vec3 {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}
