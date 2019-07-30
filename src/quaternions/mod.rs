///A purely rotation Quaternion formed by a scalar and a vector.
use super::Mat4;
use super::Vec3;
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

    pub fn slerp(self, other: Quaternion, t: f32) -> Self {
        let mut omega_cos = self.dot(other);
        let w2: Quaternion;

        if omega_cos < 0.0 {
            w2 = -other;
            omega_cos = -omega_cos;
        } else {
            w2 = other;
        }

        let k0: f32;
        let k1: f32;

        if omega_cos > 0.9999 {
            k0 = 1.0 - t;
            k1 = t;
        } else {
            let omega_sin = (1.0 - omega_cos * omega_cos).sqrt();
            let omega = omega_sin.atan2(omega_cos);

            let inverse = 1.0 / omega_sin;

            k0 = ((1.0 - t) * omega).sin() * inverse;
            k1 = (t * omega).sin() * inverse;
        }

        Quaternion {
            w: self.w * k0 + w2.w * k1,
            v: Vec3::new(
                self.v.x * k0 + w2.v.x * k1,
                self.v.y * k0 + w2.v.y * k1,
                self.v.z * k0 + w2.v.z * k1,
            ),
        }
    }

    pub fn into_mat4(self) -> Mat4 {
        let x = self.v.x;
        let y = self.v.y;
        let z = self.v.z;
        let w = self.w;

        Mat4 {
            mat: [
                [
                    1.0 - 2.0 * y.powi(2) - 2.0 * z.powi(2),
                    2.0 * x * y + 2.0 * w * z,
                    2.0 * x * z - 2.0 * w * y,
                    0.0,
                ],
                [
                    2.0 * x * y - 2.0 * w * z,
                    1.0 - 2.0 * x.powi(2) - 2.0 * z.powi(2),
                    2.0 * y * z + 2.0 * w * x,
                    0.0,
                ],
                [
                    2.0 * x * z + 2.0 * w * y,
                    2.0 * y * z - 2.0 * w * x,
                    1.0 - 2.0 * x.powi(2) - 2.0 * y.powi(2),
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
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
