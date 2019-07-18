#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use crate::vectors::Vec2;
use crate::vectors::Vec4;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    #[inline]
    pub fn magnitude(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    #[inline]
    pub fn squared_len(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
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
        let k = 1.0 / self.magnitude();
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    #[inline]
    pub fn dot(self, rhs: Vec3) -> f32 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    #[inline]
    pub fn cross(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: -(self.x * b.z - self.z * b.x),
            z: self.x * b.y - self.y * b.x,
        }
    }

    #[inline]
    pub fn distance_to(self, other: Vec3) -> f32 {
        {
            if is_x86_feature_detected!("sse2") {
                return unsafe { self.distance_to_sse2(other) };
            }
        }

        self.distance_to_gen(other)
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    #[target_feature(enable = "sse2")]
    unsafe fn distance_to_sse2(self, other: Vec3) -> f32 {
        self.distance_to_gen(other)
    }

    #[inline(always)]
    fn distance_to_gen(self, other: Vec3) -> f32 {
        let mut res: f32 = 0.0;
        for i in 0..3 {
            res = (other[i] - self[i]).powi(2);
        }
        res.sqrt()
    }

    #[inline]
    pub fn vector_to(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    #[inline]
    pub fn extend(self, w: f32) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }

    #[inline]
    pub fn truncate(self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl_vec_ops!(Vec3, x, y, z = 0, 1, 2);
