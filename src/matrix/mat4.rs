use crate::vectors::Vec4;

use std::default::Default;
use std::ops::*;

#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mat4 {
    pub mat: [[f32; 4]; 4],
}

#[allow(dead_code)]
impl Mat4 {
    #[inline]
    pub fn new(
        s0e0: f32,
        s0e1: f32,
        s0e2: f32,
        s0e3: f32,
        s1e0: f32,
        s1e1: f32,
        s1e2: f32,
        s1e3: f32,
        s2e0: f32,
        s2e1: f32,
        s2e2: f32,
        s2e3: f32,
        s3e0: f32,
        s3e1: f32,
        s3e2: f32,
        s3e3: f32,
    ) -> Self {
        Mat4 {
            mat: [
                [s0e0, s0e1, s0e2, s0e3],
                [s1e0, s1e1, s1e2, s1e3],
                [s2e0, s2e1, s2e2, s2e3],
                [s3e0, s3e1, s3e2, s3e3],
            ],
        }
    }

    #[inline]
    pub fn from_array(arr: &[[f32; 4]; 4]) -> Self {
        Mat4 { mat: *arr }
    }

    #[inline]
    pub fn from_vec(f: &Vec4, s: &Vec4, t: &Vec4, l: &Vec4) -> Self {
        Mat4 {
            mat: [
                [f.x, f.y, f.z, f.w],
                [s.x, s.y, s.z, s.w],
                [t.x, t.y, t.z, t.w],
                [l.x, l.y, l.z, l.w],
            ],
        }
    }

    #[inline]
    pub fn transpose(&mut self) {
        let mut temp = Mat4::default();

        for i in 0..3 {
            for j in 0..3 {
                temp[i][j] = self[j][i]
            }
        }
        *self = temp;
    }

    #[inline]
    pub fn transpost(&self) -> Self {
        let mut ret = Mat4::default();

        for i in 0..3 {
            for j in 0..3 {
                ret[i][j] = self[j][i];
            }
        }
        ret
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Mat4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
}

impl Index<usize> for Mat4 {
    type Output = [f32; 4];

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.mat[index]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        &mut self.mat[index]
    }
}

impl Mul<Mat4> for f32 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let mut ret = Mat4::default();

        for i in 0..3 {
            for j in 0..3 {
                ret[i][j] = rhs[i][j] * self;
            }
        }

        ret
    }
}
