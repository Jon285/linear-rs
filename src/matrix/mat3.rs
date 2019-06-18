use crate::vectors::Vec3;

use std::default::Default;
use std::ffi::c_void;
use std::ops::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3 {
    mat: [[f32; 3]; 3],
}

#[allow(dead_code)]
impl Mat3 {
    #[inline]
    pub fn new(
        s0e0: f32,
        s0e1: f32,
        s0e2: f32,
        s1e0: f32,
        s1e1: f32,
        s1e2: f32,
        s2e0: f32,
        s2e1: f32,
        s2e2: f32,
    ) -> Self {
        Mat3 {
            mat: [[s0e0, s0e1, s0e2], [s1e0, s1e1, s1e2], [s2e0, s2e1, s2e2]],
        }
    }

    #[inline]
    pub fn from_array(arr: &[[f32; 3]; 3]) -> Self {
        Mat3 { mat: *arr }
    }

    #[inline]
    pub fn from_vec(f: Vec3, s: Vec3, t: Vec3) -> Self {
        Mat3 {
            mat: [[f.x, f.y, f.z], [s.x, s.y, s.z], [t.x, t.y, t.z]],
        }
    }

    #[inline]
    pub fn transpose(&mut self) {
        let temp = *self;
        *self = Mat3 {
            mat: [
                [temp.mat[0][0], temp.mat[1][0], temp.mat[2][0]],
                [temp.mat[1][0], temp.mat[1][1], temp.mat[2][1]],
                [temp.mat[0][2], temp.mat[1][2], temp.mat[2][2]],
            ],
        }
    }

    #[inline]
    pub fn transpost(&self) -> Self {
        Mat3 {
            mat: [
                [self.mat[0][0], self.mat[1][0], self.mat[2][0]],
                [self.mat[1][0], self.mat[1][1], self.mat[2][1]],
                [self.mat[0][2], self.mat[1][2], self.mat[2][2]],
            ],
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const f32 {
        &self.mat[0][0]
    }

    #[inline]
    pub fn as_c_ptr(&self) -> *const c_void {
        &self.mat[0][0] as *const f32 as *const c_void
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: rhs.x * self.mat[0][0] + rhs.y * self.mat[1][0] + rhs.z * self.mat[2][0],
            y: rhs.x * self.mat[0][1] + rhs.y * self.mat[1][1] + rhs.z * self.mat[2][1],
            z: rhs.x * self.mat[0][2] + rhs.y * self.mat[1][2] + rhs.z * self.mat[2][2],
        }
    }
}

impl Mul<f32> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: f32) -> Mat3 {
        Mat3::new(
            self.mat[0][0] * rhs,
            self.mat[0][1] * rhs,
            self.mat[0][2] * rhs,
            self.mat[1][0] * rhs,
            self.mat[1][1] * rhs,
            self.mat[1][2] * rhs,
            self.mat[2][0] * rhs,
            self.mat[2][1] * rhs,
            self.mat[2][2] * rhs,
        )
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Self;

    fn mul(self, other: Mat3) -> Self {
        Mat3 {
            mat: [
                [
                    self[0][0] * other[0][0] + self[1][0] * other[0][1] + self[2][0] * other[0][2],
                    self[0][1] * other[0][0] + self[1][1] * other[0][1] + self[2][1] * other[0][2],
                    self[0][2] * other[0][0] + self[1][2] * other[0][1] + self[2][2] * other[0][2],
                ],
                [
                    self[0][0] * other[1][0] + self[1][0] * other[1][1] + self[2][0] * other[1][2],
                    self[0][1] * other[1][0] + self[1][1] * other[1][1] + self[2][1] * other[1][2],
                    self[0][2] * other[1][0] + self[1][2] * other[1][1] + self[2][2] * other[1][2],
                ],
                [
                    self[0][0] * other[2][0] + self[1][0] * other[2][1] + self[2][0] * other[2][2],
                    self[0][1] * other[2][0] + self[1][1] * other[2][1] + self[2][1] * other[2][2],
                    self[0][2] * other[2][0] + self[1][2] * other[2][1] + self[2][2] * other[2][2],
                ],
            ],
        }
    }
}

impl Index<usize> for Mat3 {
    type Output = [f32; 3];

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        match index {
            _ => &self.mat[index],
        }
    }
}

impl IndexMut<usize> for Mat3 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [f32; 3] {
        match index {
            _ => &mut self.mat[index],
        }
    }
}

impl Default for Mat3 {
    fn default() -> Self {
        Mat3 {
            mat: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
}
