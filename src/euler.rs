use std::convert::From;
use std::f32::consts::FRAC_PI_2;

use super::Mat3;
use super::Mat4;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Euler {
    pub yaw: f32,
    pub pitch: f32,
    pub row: f32,
}

#[allow(dead_code)]
impl Euler {
    pub fn new(yaw: f32, pitch: f32, row: f32) -> Self {
        Euler { yaw, pitch, row }
    }
}

impl From<Mat3> for Euler {
    fn from(mat: Mat3) -> Self {
        let mut ret = Self::default();

        let sp = -mat[1][2];
        if sp <= -1.0 {
            ret.pitch = -FRAC_PI_2;
        } else if sp >= 1.0 {
            ret.pitch = FRAC_PI_2;
        } else {
            ret.pitch = sp.asin();
        }

        if sp.abs() > 0.9999 {
            ret.row = 0.0;
            ret.yaw = -mat[2][1].atan2(mat[1][1]);
        } else {
            ret.row = mat[1][0].atan2(mat[1][1]);
            ret.yaw = mat[1][2].atan2(mat[2][2]);
        }
        ret
    }
}

impl From<Mat4> for Euler {
    fn from(mat: Mat4) -> Self {
        let mut ret = Self::default();

        let sp = -mat[1][2];
        if sp <= -1.0 {
            ret.pitch = -FRAC_PI_2;
        } else if sp >= 1.0 {
            ret.pitch = FRAC_PI_2;
        } else {
            ret.pitch = sp.asin();
        }

        if sp.abs() > 0.9999 {
            ret.row = 0.0;
            ret.yaw = -mat[2][1].atan2(mat[1][1]);
        } else {
            ret.row = mat[1][0].atan2(mat[1][1]);
            ret.yaw = mat[1][2].atan2(mat[2][2]);
        }
        ret
    }
}
