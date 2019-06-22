use std::default::Default;
use std::ffi::c_void;
use std::ops::*;

#[repr(C)]
#[derive(Debug)]
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

impl Index<usize> for Mat2 {
    type Output = [f32; 2];

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.mat[index]
    }
}

impl Default for Mat2 {
    fn default() -> Self {
        Mat2 {
            mat: [[1.0, 0.0], [0.0, 1.0]],
        }
    }
}
