use crate::vectors::Vec2;

use std::default::Default;
use std::ffi::c_void;
use std::ops::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat2 {
    mat: [[f32; 2]; 2],
}

#[allow(dead_code)]
impl Mat2 {
    #[inline]
    pub fn new(s0e0: f32, s0e1: f32, s1e0: f32, s1e1: f32) -> Self {
        Mat2 {
            mat: [[s0e0, s0e1], [s1e0, s1e1]],
        }
    }

    #[inline]
    pub fn from_array(arr: &[[f32; 2]; 2]) -> Self {
        Mat2 { mat: *arr }
    }

    #[inline]
    pub fn from_vec(vec: &Vec2) -> Self {
        Mat2 {
            mat: [[vec.x, vec.y], [vec.x, vec.y]],
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const f32 {
        &self[0][0] as *const f32
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        &mut self[0][0] as *mut f32
    }

    #[inline]
    pub fn as_c_void(&self) -> *const c_void {
        &self[0][0] as *const f32 as *const c_void
    }
}

impl Add<Mat2> for Mat2 {
    type Output = Self;

    fn add(self, other: Mat2) -> Self {
        Mat2 {
            mat: [
                [self[0][0] + other[0][0], self[0][1] + other[0][1]],
                [self[1][0] + other[1][0], self[1][1] + other[1][1]],
            ],
        }
    }
}

impl Sub<Mat2> for Mat2 {
    type Output = Self;

    fn sub(self, other: Mat2) -> Self {
        Mat2 {
            mat: [
                [self[0][0] - other[0][0], self[0][1] - other[0][1]],
                [self[1][0] - other[1][0], self[1][1] - other[0][1]],
            ],
        }
    }
}

impl Mul<f32> for Mat2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Mat2 {
            mat: [
                [self[0][0] * rhs, self[0][1] * rhs],
                [self[1][0] * rhs, self[1][1] * rhs],
            ],
        }
    }
}

impl Index<usize> for Mat2 {
    type Output = [f32; 2];

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.mat[index]
    }
}

impl IndexMut<usize> for Mat2 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [f32; 2] {
        &mut self.mat[index]
    }
}

impl Default for Mat2 {
    fn default() -> Self {
        Mat2 {
            mat: [[1.0, 0.0], [0.0, 1.0]],
        }
    }
}
