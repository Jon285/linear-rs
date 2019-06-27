use crate::vectors::Vec4;

use std::default::Default;
use std::ops::*;

#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mat4 {
    mat: [[f32; 4]; 4],
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

        for i in 0..=3 {
            for j in 0..=3 {
                temp[i][j] = self[j][i]
            }
        }
        *self = temp;
    }

    #[inline]
    pub fn transpost(&self) -> Self {
        let mut ret = Mat4::default();

        for i in 0..=3 {
            for j in 0..=3 {
                ret[i][j] = self[j][i];
            }
        }
        ret
    }

    #[inline]
    pub fn as_ptr(&self) -> *const f32 {
        &self[0][0] as *const f32
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        &mut self[0][0] as *mut f32
    }
}

impl_mat_ops!(Mat4, mat, 4, [f32; 4]);
impl_mat_ops!(Mat4, Vec4, 4);

impl Default for Mat4 {
    fn default() -> Self {
        Mat4::new(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        )
    }
}
