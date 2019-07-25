use super::Vec3;
///A purely rotation Quaternion formed by a scalar and a vector.
use std::ops::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub w: f32,
    pub v: Vec3,
}

#[allow(dead_code)]
impl Quaternion {
    ///Constructs a new Quaternion with a angle and a axis of rotation
    pub fn new(ang: f32, axis: Vec3) -> Self {
        let axis = axis.normalized();
        let div = ang / 2.0;

        Quaternion {
            w: div.cos(),
            v: div.sin() * axis,
        }
    }

    #[inline]
    pub fn magnitude(self) -> f32 {
        (self.w.powi(2) + self.v.magnitude().powi(2)).sqrt()
    }

    #[inline]
    pub fn conjugate(self) -> Self {
        Quaternion {
            w: self.w,
            v: -self.v,
        }
    }

    #[inline]
    pub fn displacement_from(self, other: Quaternion) -> Self {
        other * self.conjugate()
    }

    #[inline]
    pub fn dot(self, other: Quaternion) -> f32 {
        self.w * other.w + self.v.dot(other.v)
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Self;

    fn mul(self, other: Quaternion) -> Self::Output {
        Quaternion {
            w: self.w * other.w - self.v.dot(other.v),
            v: self.w * other.v + other.w * self.v + self.v.cross(other.v),
        }
    }
}
