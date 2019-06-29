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
    pub fn zero() -> Self {
        Mat2 {
            mat: [[0.0, 0.0], [0.0, 0.0]],
        }
    }

    //====================================== TRANSFORMATION MATRICES ===============================

    ///Returns a rotation Matrix around the origin
    #[inline]
    pub fn rotation(ang: f32) -> Self {
        Mat2 {
            mat: [[ang.cos(), -ang.sin()], [ang.sin(), ang.cos()]],
        }
    }

    ///Uniform scale Matrix in all directions by a factor `k`
    #[inline]
    pub fn scale(k: f32) -> Self {
        Mat2 {
            mat: [[k, 0.0], [0.0, k]],
        }
    }

    ///Arbitrary scale Matrix towards `n` by a factor of `k`
    #[inline]
    pub fn scale_arb(k: f32, n: &Vec2) -> Self {
        let n = n.normalized();

        Mat2 {
            mat: [
                [1.0 + (k - 1.0) * n.x.powi(2), (k - 1.0) * n.x * n.y],
                [(k - 1.0) * n.x * n.y, 1.0 + (k - 1.0) * n.y.powi(2)],
            ],
        }
    }

    ///Projection Matrix in the x-axis
    #[inline]
    pub fn projection_x() -> Self {
        Mat2 {
            mat: [[1.0, 0.0], [0.0, 0.0]],
        }
    }

    ///Projection Matrix in the y-axis
    #[inline]
    pub fn projection_y() -> Self {
        Mat2 {
            mat: [[0.0, 0.0], [0.0, 1.0]],
        }
    }

    ///Create a projection Matrix in the arbitrary `n` axis
    #[inline]
    pub fn projection(n: &Vec2) -> Self {
        let n = n.normalized();

        Mat2 {
            mat: [
                [1.0 - n.x.powi(2), -n.x * n.y],
                [-n.x * n.y, 1.0 - n.y.powi(2)],
            ],
        }
    }

    ///Reflection Matrix about the `n` axis
    #[inline]
    pub fn reflection(n: &Vec2) -> Self {
        let n = n.normalized();

        Mat2 {
            mat: [
                [1.0 - 2.0 * n.x.powi(2), -2.0 * n.x * n.y],
                [-2.0 * n.x * n.y, 1.0 - 2.0 * n.y.powi(2)],
            ],
        }
    }

    #[inline]
    pub fn shearing_x(s: f32) -> Self {
        Mat2 {
            mat: [[1.0, 0.0], [s, 1.0]],
        }
    }

    #[inline]
    pub fn shearing_y(s: f32) -> Self {
        Mat2 {
            mat: [[1.0, s], [0.0, 0.0]],
        }
    }

    //====================================================================================

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

impl Default for Mat2 {
    fn default() -> Self {
        Mat2 {
            mat: [[1.0, 0.0], [0.0, 1.0]],
        }
    }
}
