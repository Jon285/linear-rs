use crate::vectors::Vec2;

use std::default::Default;
use std::ffi::c_void;

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
    pub fn rotation(ang: f32) -> Self {
        Mat2 {
            mat: [[ang.cos(), -ang.sin()], [ang.sin(), ang.cos()]],
        }
    }

    #[inline]
    pub fn scale(k: f32) -> Self {
        Mat2 {
            mat: [[k, 0.0], [0.0, k]],
        }
    }

    #[inline]
    pub fn transpost(&self) -> Self {
        Mat2 {
            mat: [[self[0][0], self[1][0]], [self[0][1], self[1][1]]],
        }
    }

    #[inline]
    pub fn transpose(&mut self) {
        *self = Mat2 {
            mat: [[self[0][0], self[1][0]], [self[0][1], self[1][1]]],
        };
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

impl_mat_ops!(Mat2, mat, 2, [f32; 2]);
impl_mat_ops!(Mat2, Vec2, 2);

//impl Mul<Vec2> for Mat2 {
//    type Output = Vec2;
//
//    fn mul(self, rhs: Vec2) -> Self::Output {
//        Vec2 {
//            x: self[0][0] * rhs.x + self[1][0] * rhs.y,
//            y: self[0][1] * rhs.x + self[1][1] * rhs.y,
//        }
//    }
//}

impl Default for Mat2 {
    fn default() -> Self {
        Mat2 {
            mat: [[1.0, 0.0], [0.0, 1.0]],
        }
    }
}
