use num_traits::identities;

use crate::vectors::Vec2;
use crate::RealScalar;

use std::convert::From;
use std::default::Default;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat2<T> {
    mat: [[T; 2]; 2],
}

#[allow(dead_code)]
impl<T> Mat2<T> {
    #[inline]
    pub fn new(s0e0: T, s0e1: T, s1e0: T, s1e1: T) -> Self {
        Mat2 {
            mat: [[s0e0, s0e1], [s1e0, s1e1]],
        }
    }
}

impl<T: RealScalar> Mat2<T> {
    #[inline]
    pub fn zero() -> Self {
        Mat2 {
            mat: [
                [identities::zero::<T>(), identities::zero::<T>()],
                [identities::zero::<T>(), identities::zero::<T>()],
            ],
        }
    }

    //====================================== TRANSFORMATION MATRICES ===============================

    ///Returns a rotation Matrix around the origin
    #[inline]
    pub fn rotation(ang: T) -> Self {
        Mat2 {
            mat: [[ang.cos(), -ang.sin()], [ang.sin(), ang.cos()]],
        }
    }

    ///Uniform scale Matrix in all directions by a factor `k`
    #[inline]
    pub fn scale(k: T) -> Self {
        Mat2 {
            mat: [[k, 0.0], [0.0, k]],
        }
    }

    ///Arbitrary scale Matrix towards `n` by a factor of `k`
    #[inline]
    pub fn scale_arb(k: T, n: Vec2<T>) -> Self {
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
    pub fn projection(n: Vec2<T>) -> Self {
        let n = n.normalized();
        let one = identities::one::<T>();

        Mat2 {
            mat: [
                [one - n.x.powi(2), -n.x * n.y],
                [-n.x * n.y, one - n.y.powi(2)],
            ],
        }
    }

    ///Reflection Matrix about the `n` axis
    #[inline]
    pub fn reflection(n: Vec2<T>) -> Self {
        let n = n.normalized();

        Mat2 {
            mat: [
                [1.0 - 2.0 * n.x.powi(2), -2.0 * n.x * n.y],
                [-2.0 * n.x * n.y, 1.0 - 2.0 * n.y.powi(2)],
            ],
        }
    }

    #[inline]
    pub fn shearing_x(s: T) -> Self {
        Mat2 {
            mat: [[1.0, 0.0], [s, 1.0]],
        }
    }

    #[inline]
    pub fn shearing_y(s: T) -> Self {
        Mat2 {
            mat: [[1.0, s], [0.0, 0.0]],
        }
    }

    //====================================================================================

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
    pub fn determinant(&self) -> T {
        self[0][0] * self[1][1] - self[1][0] * self[0][1]
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        &self[0][0] as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        &mut self[0][0] as *mut T
    }
}

// impl_mat_ops!(Mat2, mat, 2, [T; 2]);
// impl_mat_ops!(Mat2, Vec2, 2);

impl<T> Default for Mat2<T> {
    fn default() -> Self {
        Mat2 {
            mat: [
                [identities::one::<T>(), identities::zero::<T>()],
                [identities::zero::<T>(), identities::one::<T>()],
            ],
        }
    }
}

impl<T> From<[[T; 2]; 2]> for Mat2<T> {
    fn from(array: [[T; 2]; 2]) -> Self {
        Mat2 { mat: array }
    }
}

impl<T> From<(Vec2<T>, Vec2<T>)> for Mat2<T> {
    fn from(tuple: (Vec2<T>, Vec2<T>)) -> Self {
        Mat2 {
            mat: [[tuple.0.x, tuple.0.y], [tuple.1.x, tuple.1.y]],
        }
    }
}
