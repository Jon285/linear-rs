use num_traits::identities;

use std::convert::From;
use std::default::Default;

use crate::euler::Euler;
use crate::matrix::Mat2;
use crate::quaternions::Quaternion;
use crate::vectors::Vec3;
use crate::{FloatScalar, RealScalar};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3<T> {
    mat: [[T; 3]; 3],
}

#[allow(dead_code)]
impl<T> Mat3<T> {
    #[inline]
    pub const fn new(
        s0e0: T,
        s0e1: T,
        s0e2: T,
        s1e0: T,
        s1e1: T,
        s1e2: T,
        s2e0: T,
        s2e1: T,
        s2e2: T,
    ) -> Self {
        Mat3 {
            mat: [[s0e0, s0e1, s0e2], [s1e0, s1e1, s1e2], [s2e0, s2e1, s2e2]],
        }
    }
}

impl<T: RealScalar> Mat3<T> {
    ///Returns a null matrix
    pub fn zero() -> Self {
        let zero = identities::zero::<T>();
        Mat3 {
            mat: [[zero, zero, zero], [zero, zero, zero], [zero, zero, zero]],
        }
    }
}

impl<T: FloatScalar> Mat3<T> {
    //================================== TRANSFORMATION MATRICES =========================================

    ///Returns a rotation Matrix around the x-axis
    #[inline]
    pub fn rotation_x(ang: T) -> Self {
        let zero = identities::zero::<T>();
        let one: T = identities::one::<T>();
        Mat3 {
            mat: [
                [one, zero, zero],
                [zero, ang.cos(), ang.sin()],
                [zero, -ang.sin(), ang.cos()],
            ],
        }
    }

    ///Returns a rotation Matrix around the y-axis
    #[inline]
    pub fn rotation_y(ang: T) -> Self {
        let zero = identities::zero::<T>();
        let one: T = identities::one::<T>();
        Mat3 {
            mat: [
                [ang.cos(), zero, -ang.sin()],
                [zero, one, zero],
                [ang.sin(), zero, ang.cos()],
            ],
        }
    }

    ///Returns a rotation Matrix around the z-axis
    #[inline]
    pub fn rotation_z(ang: T) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();
        let zero = identities::zero::<T>();
        let one: T = identities::one::<T>();

        Mat3 {
            mat: [[cos, sin, zero], [-sin, cos, zero], [zero, zero, one]],
        }
    }

    ///Returns a projection Matrix in the x- and y-axis
    #[inline]
    pub fn projection_xy() -> Self {
        let zero = identities::zero::<T>();
        let one: T = identities::one::<T>();
        Mat3 {
            mat: [[one, zero, zero], [zero, one, zero], [zero, zero, zero]],
        }
    }

    ///Returns a projection Matrix in the x- and z-axis
    #[inline]
    pub fn projection_xz() -> Self {
        let zero = identities::zero::<T>();
        let one: T = identities::one::<T>();
        Mat3 {
            mat: [[one, zero, zero], [zero, zero, zero], [zero, zero, one]],
        }
    }

    ///Returns a projection Matrix in the y- and z-axis
    #[inline]
    pub fn projection_yz() -> Self {
        let zero = identities::zero::<T>();
        let one: T = identities::one::<T>();
        Mat3 {
            mat: [[zero, zero, zero], [zero, one, zero], [zero, zero, one]],
        }
    }

    ///Creates a projection Matrix in the plane perpendicular to the Vector `n`
    #[inline]
    pub fn projection(n: Vec3<T>) -> Self {
        let n = n.normalized();
        let one: T = identities::one::<T>();

        Mat3 {
            mat: [
                [one - n.x.powi(2), -n.x * n.y, -n.x * n.z],
                [-n.x * n.y, one - n.y.powi(2), -n.y * n.z],
                [-n.x * n.z, -n.y * n.z, one - n.z.powi(2)],
            ],
        }
    }

    ///Creates a rotation Matrix around de axis of `n` by `ang` radians
    #[inline]
    pub fn rotation(ang: T, n: Vec3<T>) -> Self {
        let n = n.normalized();

        let one = identities::one::<T>();
        let factor = one - ang.cos();
        let sin = ang.sin();
        let cos = ang.cos();

        Mat3 {
            mat: [
                [
                    n.x.powi(2) * factor + cos,
                    n.x * n.y * factor + n.z * sin,
                    n.x * n.z * factor - n.y * sin,
                ],
                [
                    n.x * n.y * factor - n.z * sin,
                    n.y.powi(2) * factor + cos,
                    n.y * n.z * factor + n.x * sin,
                ],
                [
                    n.x * n.z * factor + n.y * sin,
                    n.y * n.z * factor - n.x * sin,
                    n.z.powi(2) * factor + cos,
                ],
            ],
        }
    }

    ///Returns a matrix to uniformly scale a 3D vector in all directions by a factor `k`
    #[inline]
    pub fn scale(k: T) -> Self {
        let zero = identities::zero::<T>();
        Mat3 {
            mat: [[k, zero, zero], [zero, k, zero], [zero, zero, k]],
        }
    }

    ///Creates a scale Matrix towards the arbitrary direction of `n` by a factor of `k`
    #[inline]
    pub fn scale_arb(k: T, n: Vec3<T>) -> Self {
        let n = n.normalized();
        let one: T = identities::one::<T>();

        //pre calculating some of the members
        let scale = one + (k - one);
        let nx_ny = (k - one) * n.x * n.y;
        let nx_nz = (k - one) * n.x * n.z;
        let ny_nz = (k - one) * n.y * n.z;

        Mat3 {
            mat: [
                [scale * n.x.powi(2), nx_ny, nx_nz],
                [nx_ny, scale * n.y.powi(2), ny_nz],
                [nx_nz, ny_nz, scale * n.z.powi(2)],
            ],
        }
    }

    #[inline]
    pub fn reflection(n: Vec3<T>) -> Self {
        let n = n.normalized();
        let one: T = identities::one::<T>();
        let two = one + one;

        Mat3 {
            mat: [
                [one - two * n.x.powi(2), -two * n.x * n.y, -two * n.x * n.z],
                [-two * n.x * n.y, one - two * n.y.powi(2), -two * n.x * n.z],
                [-two * n.x * n.z, -two * n.y * n.z, one - two * n.z.powi(2)],
            ],
        }
    }

    #[inline]
    pub fn shearing_xy(s: T, t: T) -> Self {
        let zero = identities::zero::<T>();
        let one: T = identities::one::<T>();
        Mat3 {
            mat: [[one, zero, zero], [zero, one, zero], [s, t, one]],
        }
    }

    #[inline]
    pub fn shearing_xz(s: T, t: T) -> Self {
        let zero = identities::zero::<T>();
        let one: T = identities::one::<T>();
        Mat3 {
            mat: [[one, zero, zero], [s, one, t], [zero, zero, one]],
        }
    }

    #[inline]
    pub fn shearing_yz(s: T, t: T) -> Self {
        let zero = identities::zero::<T>();
        let one: T = identities::one::<T>();
        Mat3 {
            mat: [[one, s, t], [zero, one, zero], [zero, zero, one]],
        }
    }

    //=============================================================================================================

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

    pub fn minor(&self, i: usize, j: usize) -> T {
        if i > 2 || j > 2 {
            panic!("out of bonds matrix access");
        }

        let mut matrix_2 = Mat2::default();
        let mut col = 0;
        let mut row = 0;

        for a in 0..3 {
            for b in 0..3 {
                //are we in the excluded row or column?
                if a != i && b != j {
                    matrix_2[col][row] = self[a][b];
                    row += 1;

                    //column is filled, change to the next and reset the row
                    if row == 2 {
                        row = 0;
                        col += 1;
                    }
                }
            }
        }
        matrix_2.determinant()
    }

    pub fn cofactor(&self, i: usize, j: usize) -> T {
        let one = identities::one::<T>();
        let sign = if (i + j) % 2 == 0 { one } else { -one };

        sign * self.minor(i, j)
    }

    #[inline]
    pub fn determinant(&self) -> T {
        let zero = identities::zero::<T>();
        let mut ret: T = zero;

        //hardcoded column value since the result is the same in any other
        for i in 0..3 {
            ret += self[0][i] * self.cofactor(0, i);
        }
        ret
    }

    pub fn inverse(&self) -> Option<Mat3<T>> {
        let determinant = self.determinant();
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        if determinant == zero {
            return None;
        }

        let div = one / determinant;
        let mut adj = Mat3::default();

        //create the adjoint matrix
        for i in 0..3 {
            for j in 0..3 {
                adj[j][i] = self.cofactor(i, j);
            }
        }
        Some(adj * div)
    }

    #[inline]
    pub fn as_ptr(&self) -> *const T {
        &self.mat[0][0] as *const T
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        &mut self[0][0] as *mut T
    }
}

impl_mat_ops!(Mat3, mat, 3, [T; 3]);
impl_mat_ops!(Mat3, Vec3, 3);

impl<T: RealScalar> Default for Mat3<T> {
    fn default() -> Self {
        let zero = identities::zero::<T>();
        let one: T = identities::one::<T>();
        Mat3 {
            mat: [[one, zero, zero], [zero, one, zero], [zero, zero, one]],
        }
    }
}
impl<T: FloatScalar> From<Quaternion<T>> for Mat3<T> {
    fn from(quat: Quaternion<T>) -> Self {
        let x = quat.v.x;
        let y = quat.v.y;
        let z = quat.v.z;
        let w = quat.w;
        let one = identities::one::<T>();
        let two = one + one;

        Mat3 {
            mat: [
                [
                    one - two * y.powi(2) - two * z.powi(2),
                    two * x * y + two * w * z,
                    two * x * z - two * w * y,
                ],
                [
                    two * x * y - two * w * z,
                    one - two * x.powi(2) - two * z.powi(2),
                    two * y * z + two * w * x,
                ],
                [
                    two * x * z + two * w * y,
                    two * y * z - two * w * x,
                    one - two * x.powi(2) - two * y.powi(2),
                ],
            ],
        }
    }
}

impl<T: FloatScalar> From<Euler<T>> for Mat3<T> {
    fn from(euler: Euler<T>) -> Self {
        let cy = euler.yaw.cos();
        let cp = euler.pitch.cos();
        let cr = euler.row.cos();
        let sy = euler.yaw.cos();
        let sp = euler.pitch.cos();
        let sr = euler.row.sin();

        Mat3 {
            mat: [
                [cy * cr + sy * sp * sr, sr * cp, -sy * cr + cy * sp * sr],
                [-cy * sr + sy * sp * cr, cr * cp, sr * sy + cy * sp * cr],
                [sy * cp, -sp, cy * cp],
            ],
        }
    }
}

impl<T> From<[[T; 3]; 3]> for Mat3<T> {
    fn from(array: [[T; 3]; 3]) -> Self {
        Mat3 { mat: array }
    }
}

impl<T> From<(Vec3<T>, Vec3<T>, Vec3<T>)> for Mat3<T> {
    fn from(tuple: (Vec3<T>, Vec3<T>, Vec3<T>)) -> Self {
        Mat3 {
            mat: [
                [tuple.0.x, tuple.0.y, tuple.0.z],
                [tuple.1.x, tuple.1.y, tuple.1.z],
                [tuple.2.x, tuple.2.y, tuple.2.z],
            ],
        }
    }
}
