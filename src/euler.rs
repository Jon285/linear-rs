use num_traits::cast;
use num_traits::float::FloatConst;
use num_traits::identities;

use std::convert::From;

use super::FloatScalar;
use super::Mat3;
use super::Mat4;
use super::Quaternion;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Euler<T: FloatScalar> {
    pub yaw: T,
    pub pitch: T,
    pub row: T,
}

#[allow(dead_code)]
impl<T: FloatScalar> Euler<T> {
    pub fn new(yaw: T, pitch: T, row: T) -> Self {
        Euler { yaw, pitch, row }
    }
}

impl<T: FloatScalar + FloatConst> From<Mat3<T>> for Euler<T> {
    fn from(mat: Mat3<T>) -> Self {
        let mut ret = Self::default();
        let one = identities::one::<T>();

        let sp = -mat[1][2];
        if sp <= -one {
            ret.pitch = -T::FRAC_PI_2();
        } else if sp >= one {
            ret.pitch = T::FRAC_PI_2();
        } else {
            ret.pitch = sp.asin();
        }

        if sp.abs() > cast::cast::<f64, T>(0.9999).unwrap() {
            ret.row = identities::zero::<T>();
            ret.yaw = -mat[2][1].atan2(mat[1][1]);
        } else {
            ret.row = mat[1][0].atan2(mat[1][1]);
            ret.yaw = mat[1][2].atan2(mat[2][2]);
        }
        ret
    }
}

impl<T: FloatScalar + FloatConst> From<Mat4<T>> for Euler<T> {
    fn from(mat: Mat4<T>) -> Self {
        let mut ret = Self::default();
        let one = identities::one::<T>();

        let sp = -mat[1][2];
        if sp <= -one {
            ret.pitch = -T::FRAC_PI_2();
        } else if sp >= one {
            ret.pitch = T::FRAC_PI_2();
        } else {
            ret.pitch = sp.asin();
        }

        if sp.abs() > cast::cast::<f64, T>(0.9999).unwrap() {
            ret.row = identities::zero::<T>();
            ret.yaw = -mat[2][1].atan2(mat[1][1]);
        } else {
            ret.row = mat[1][0].atan2(mat[1][1]);
            ret.yaw = mat[1][2].atan2(mat[2][2]);
        }
        ret
    }
}

impl<T: FloatScalar + FloatConst> From<Quaternion<T>> for Euler<T> {
    fn from(quat: Quaternion<T>) -> Self {
        let one = identities::one::<T>();
        let sp = -(one + one) * (quat.v.y * quat.v.z - quat.w * quat.v.x);

        let half = cast::cast::<f64, T>(0.5).unwrap();
        if sp.abs() > cast::cast::<f64, T>(0.99999).unwrap() {
            Euler {
                yaw: (-quat.v.x * quat.v.z + quat.w * quat.v.y)
                    .atan2(half - quat.v.y.powi(2) - quat.v.z.powi(2)),
                pitch: T::FRAC_PI_2() * sp,
                row: identities::zero::<T>(),
            }
        } else {
            Euler {
                pitch: sp.asin(),
                yaw: (quat.v.x * quat.v.z + quat.w * quat.v.y)
                    .atan2(half - quat.v.x.powi(2) - quat.v.y.powi(2)),
                row: (quat.v.y * quat.v.x + quat.w * quat.v.z)
                    .atan2(half - quat.v.x.powi(2) - quat.v.z.powi(2)),
            }
        }
    }
}
