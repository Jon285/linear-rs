use std::convert::From;
use std::ops::*;

use super::Euler;
use super::Mat3;
use super::Mat4;
use super::Vec3;

///A purely rotation Quaternion formed by a scalar and a vector.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Quaternion {
    pub w: f32,
    pub v: Vec3,
}

#[allow(dead_code)]
impl Quaternion {
    ///Constructs a new rotation Quaternion with a angle and a axis of rotation
    #[inline]
    pub fn new(ang: f32, axis: Vec3) -> Self {
        let axis = axis.normalized();
        let div = ang / 2.0;

        Quaternion {
            w: div.cos(),
            v: div.sin() * axis,
        }
    }

    ///Constructs a new standard Quaternion with the passed scalar and vector
    #[inline]
    pub fn new_sv(w: f32, v: Vec3) -> Self {
        Quaternion { w, v }
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

    ///Returns a Quaternion that corresponds to the displacement between `self` and `other`
    #[inline]
    pub fn displacement_from(self, other: Quaternion) -> Self {
        other * self.conjugate()
    }

    #[inline]
    pub fn dot(self, other: Quaternion) -> f32 {
        self.w * other.w + self.v.dot(other.v)
    }

    ///Raises self to the power of `exp`
    pub fn pow(self, exp: f32) -> Self {
        if self.w.abs() > 0.99999 {
            return self;
        }

        let alpha = self.w.acos() * exp;
        let w = alpha.cos();

        let mult = alpha.sin() / self.w.acos().sin();

        Quaternion {
            w,
            v: Vec3::new(self.v.x * mult, self.v.y * mult, self.v.z * mult),
        }
    }

    ///Calculates the spherical interpolation between `self` and `other` by the amount of `t`
    pub fn slerp(self, other: Quaternion, t: f32) -> Self {
        let mut omega_cos = self.dot(other);
        let w2: Quaternion;

        if omega_cos < 0.0 {
            w2 = -other;
            omega_cos = -omega_cos;
        } else {
            w2 = other;
        }

        let (k0, k1) = if omega_cos > 0.9999 {
            (1.0 - t, t)
        } else {
            let omega_sin = (1.0 - omega_cos * omega_cos).sqrt();
            let omega = omega_sin.atan2(omega_cos);

            let inverse = 1.0 / omega_sin;

            (
                ((1.0 - t) * omega).sin() * inverse,
                (t * omega).sin() * inverse,
            )
        };

        Quaternion {
            w: self.w * k0 + w2.w * k1,
            v: Vec3::new(
                self.v.x * k0 + w2.v.x * k1,
                self.v.y * k0 + w2.v.y * k1,
                self.v.z * k0 + w2.v.z * k1,
            ),
        }
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

impl Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        Quaternion {
            w: self.w * other,
            v: self.v * other,
        }
    }
}

impl Neg for Quaternion {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Quaternion {
            w: -self.w,
            v: -self.v,
        }
    }
}

#[allow(non_snake_case)]
impl From<Mat3> for Quaternion {
    fn from(mat: Mat3) -> Self {
        let mut ret = Quaternion::default();

        let W = mat[0][0] + mat[1][1] + mat[2][2];
        let X = mat[0][0] - mat[1][1] + mat[2][2];
        let Y = mat[1][1] - mat[0][0] - mat[2][2];
        let Z = mat[2][2] - mat[0][0] - mat[1][1];

        let mut index = 0;
        let mut biggest = W;

        if X > biggest {
            biggest = X;
            index = 1;
        }

        if Y > biggest {
            biggest = Y;
            index = 2;
        }

        if Z > biggest {
            biggest = Z;
            index = 3;
        }

        let largest = (biggest + 1.0).sqrt() * 0.5;
        let mult = 0.25 / largest;

        match index {
            0 => {
                ret.w = largest;
                ret.v.x = (mat[2][1] - mat[1][2]) * mult;
                ret.v.y = (mat[0][2] - mat[2][0]) * mult;
                ret.v.z = (mat[1][0] - mat[0][1]) * mult;
                ret
            }

            1 => {
                ret.v.x = largest;
                ret.w = (mat[2][1] - mat[1][2]) * mult;
                ret.v.y = (mat[1][0] + mat[0][1]) * mult;
                ret.v.z = (mat[0][2] + mat[2][0]) * mult;
                ret
            }

            2 => {
                ret.v.y = largest;
                ret.w = (mat[0][2] - mat[2][0]) * mult;
                ret.v.x = (mat[1][0] + mat[0][1]) * mult;
                ret.v.z = (mat[2][1] + mat[1][2]) * mult;
                ret
            }
            _ => ret,
        }
    }
}

#[allow(non_snake_case)]
impl From<Mat4> for Quaternion {
    fn from(mat: Mat4) -> Self {
        let mut ret = Quaternion::default();

        let W = mat[0][0] + mat[1][1] + mat[2][2];
        let X = mat[0][0] - mat[1][1] + mat[2][2];
        let Y = mat[1][1] - mat[0][0] - mat[2][2];
        let Z = mat[2][2] - mat[0][0] - mat[1][1];

        let mut index = 0;
        let mut biggest = W;

        if X > biggest {
            biggest = X;
            index = 1;
        }

        if Y > biggest {
            biggest = Y;
            index = 2;
        }

        if Z > biggest {
            biggest = Z;
            index = 3;
        }

        let largest = (biggest + 1.0).sqrt() * 0.5;
        let mult = 0.25 / largest;

        match index {
            0 => {
                ret.w = largest;
                ret.v.x = (mat[2][1] - mat[1][2]) * mult;
                ret.v.y = (mat[0][2] - mat[2][0]) * mult;
                ret.v.z = (mat[1][0] - mat[0][1]) * mult;
                ret
            }

            1 => {
                ret.v.x = largest;
                ret.w = (mat[2][1] - mat[1][2]) * mult;
                ret.v.y = (mat[1][0] + mat[0][1]) * mult;
                ret.v.z = (mat[0][2] + mat[2][0]) * mult;
                ret
            }

            2 => {
                ret.v.y = largest;
                ret.w = (mat[0][2] - mat[2][0]) * mult;
                ret.v.x = (mat[1][0] + mat[0][1]) * mult;
                ret.v.z = (mat[2][1] + mat[1][2]) * mult;
                ret
            }
            3 => {
                ret.v.z = largest;
                ret.w = (mat[1][0] - mat[0][1]) * mult;
                ret.v.x = (mat[0][2] + mat[2][0]) * mult;
                ret.v.y = (mat[2][1] + mat[1][2]) * mult;
                ret
            }
            _ => ret,
        }
    }
}

impl From<Euler> for Quaternion {
    fn from(euler: Euler) -> Self {
        let yaw = euler.yaw / 2.0;
        let pitch = euler.pitch / 2.0;
        let row = euler.row / 2.0;

        let w = yaw.cos() * pitch.cos() * row.cos() + yaw.sin() * pitch.sin() * row.sin();
        let x = yaw.cos() * pitch.sin() * row.cos() + yaw.sin() * pitch.cos() * row.sin();
        let y = yaw.sin() * pitch.cos() * row.cos() - yaw.cos() * pitch.sin() * row.sin();
        let z = yaw.cos() * pitch.cos() * row.sin() - yaw.sin() * pitch.sin() * row.cos();
        Quaternion {
            w,
            v: Vec3::new(x, y, z),
        }
    }
}
