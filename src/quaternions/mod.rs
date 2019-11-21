use num_traits::cast;
use num_traits::identities;

use std::convert::From;
use std::ops::{Mul, Neg};

use super::Euler;
use super::FloatScalar;
use super::Mat3;
use super::Mat4;
use super::Vec3;

///A purely rotation Quaternion formed by a scalar and a vector.
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Quaternion<T: FloatScalar> {
    pub w: T,
    pub v: Vec3<T>,
}

#[allow(dead_code)]
impl<T: FloatScalar> Quaternion<T> {
    ///Constructs a new rotation Quaternion with a angle and a axis of rotation
    #[inline]
    pub fn new(ang: T, axis: Vec3<T>) -> Self {
        let axis = axis.normalized();
        let one = identities::one::<T>();
        let div = ang / (one + one);

        Quaternion {
            w: div.cos(),
            v: (axis * div.sin()),
        }
    }

    ///Constructs a new standard Quaternion with the passed scalar and vector
    #[inline]
    pub fn new_sv(w: T, v: Vec3<T>) -> Self {
        Quaternion { w, v }
    }

    #[inline]
    pub fn magnitude(self) -> T {
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
    pub fn displacement_from(self, other: Quaternion<T>) -> Self {
        other * self.conjugate()
    }

    #[inline]
    pub fn dot(self, other: Quaternion<T>) -> T {
        self.w * other.w + self.v.dot(other.v)
    }

    ///Raises self to the power of `exp`
    pub fn pow(self, exp: T) -> Self {
        if self.w.abs() > cast::cast::<f64, T>(0.99999).unwrap() {
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
    pub fn slerp(self, other: Quaternion<T>, t: T) -> Self {
        let mut omega_cos = self.dot(other);
        let w2: Quaternion<T>;
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();

        if omega_cos < zero {
            w2 = -other;
            omega_cos = -omega_cos;
        } else {
            w2 = other;
        }

        let (k0, k1) = if omega_cos > cast::cast::<f64, T>(0.9999).unwrap() {
            (one - t, t)
        } else {
            let omega_sin = (one - omega_cos * omega_cos).sqrt();
            let omega = omega_sin.atan2(omega_cos);

            let inverse = one / omega_sin;

            (
                ((one - t) * omega).sin() * inverse,
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

impl<T: FloatScalar> Mul<Quaternion<T>> for Quaternion<T> {
    type Output = Self;

    fn mul(self, other: Quaternion<T>) -> Self::Output {
        Quaternion {
            w: self.w * other.w - self.v.dot(other.v),
            v: other.v * self.w + self.v + self.v.cross(other.v) * other.w,
        }
    }
}

impl<T: FloatScalar> Mul<T> for Quaternion<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Quaternion {
            w: self.w * other,
            v: self.v * other,
        }
    }
}

impl<T: FloatScalar> Neg for Quaternion<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Quaternion {
            w: -self.w,
            v: -self.v,
        }
    }
}

#[allow(non_snake_case)]
impl<T: FloatScalar> From<Mat3<T>> for Quaternion<T> {
    fn from(mat: Mat3<T>) -> Self {
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

        let largest =
            (biggest + identities::one::<T>()).sqrt() * cast::cast::<f64, T>(0.5).unwrap();
        let mult = cast::cast::<f64, T>(0.25).unwrap() / largest;

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
impl<T: FloatScalar> From<Mat4<T>> for Quaternion<T> {
    fn from(mat: Mat4<T>) -> Self {
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

        let largest =
            (biggest + identities::one::<T>()).sqrt() * cast::cast::<f64, T>(0.5).unwrap();
        let mult = cast::cast::<f64, T>(0.25).unwrap() / largest;

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

impl<T: FloatScalar> From<Euler<T>> for Quaternion<T> {
    fn from(euler: Euler<T>) -> Self {
        let two = identities::one::<T>() + identities::one::<T>();

        let yaw = euler.yaw / two;
        let pitch = euler.pitch / two;
        let row = euler.row / two;

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
