use num_traits::identities;

use std::convert::From;
use std::default::Default;

use crate::euler::Euler;
use crate::matrix::Mat2;
use crate::quaternions::Quaternion;
use crate::vectors::Vec3;
use crate::FloatScalar;

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

impl<T: FloatScalar> Mat3<T> {
    const ZERO: T = identities::zero::<T>();
    const ONE: T = identities::one::<T>();

    ///Returns a null matrix
    pub fn zero() -> Self {
        let zero = identities::zero::<T>();
        Mat3 {
            mat: [[zero, zero, zero], [zero, zero, zero], [zero, zero, zero]],
        }
    }

    //================================== TRANSFORMATION MATRICES =========================================

    ///Returns a rotation Matrix around the x-axis
    #[inline]
    pub fn rotation_x(ang: T) -> Self {
        Mat3 {
            mat: [
                [Self::ONE, Self::ZERO, Self::ZERO],
                [Self::ZERO, ang.cos(), ang.sin()],
                [Self::ZERO, -ang.sin(), ang.cos()],
            ],
        }
    }

    ///Returns a rotation Matrix around the y-axis
    #[inline]
    pub fn rotation_y(ang: T) -> Self {
        Mat3 {
            mat: [
                [ang.cos(), Self::ZERO, -ang.sin()],
                [Self::ZERO, Self::ONE, Self::ZERO],
                [ang.sin(), Self::ZERO, ang.cos()],
            ],
        }
    }

    ///Returns a rotation Matrix around the z-axis
    #[inline]
    pub fn rotation_z(ang: T) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();

        Mat3 {
            mat: [
                [cos, sin, Self::ZERO],
                [-sin, cos, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }

    ///Returns a projection Matrix in the x- and y-axis
    #[inline]
    pub fn projection_xy() -> Self {
        Mat3 {
            mat: [
                [Self::ONE, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ONE, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO],
            ],
        }
    }

    ///Returns a projection Matrix in the x- and z-axis
    #[inline]
    pub fn projection_xz() -> Self {
        Mat3 {
            mat: [
                [Self::ONE, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }

    ///Returns a projection Matrix in the y- and z-axis
    #[inline]
    pub fn projection_yz() -> Self {
        Mat3 {
            mat: [
                [Self::ZERO, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ONE, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }

    ///Creates a projection Matrix in the plane perpendicular to the Vector `n`
    #[inline]
    pub fn projection(n: Vec3<T>) -> Self {
        let n = n.normalized();

        Mat3 {
            mat: [
                [Self::ONE - n.x.powi(2), -n.x * n.y, -n.x * n.z],
                [-n.x * n.y, Self::ONE - n.y.powi(2), -n.y * n.z],
                [-n.x * n.z, -n.y * n.z, Self::ONE - n.z.powi(2)],
            ],
        }
    }

    ///Creates a rotation Matrix around de axis of `n` by `ang` radians
    #[inline]
    pub fn rotation(ang: T, n: Vec3<T>) -> Self {
        let n = n.normalized();

        let factor = Self::ONE - ang.cos();
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
        Mat3 {
            mat: [
                [k, Self::ZERO, Self::ZERO],
                [Self::ZERO, k, Self::ZERO],
                [Self::ZERO, Self::ZERO, k],
            ],
        }
    }

    ///Creates a scale Matrix towards the arbitrary direction of `n` by a factor of `k`
    #[inline]
    pub fn scale_arb(k: T, n: Vec3<T>) -> Self {
        let n = n.normalized();

        //pre calculating some of the members
        let scale = Self::ONE + (k - Self::ONE);
        let nx_ny = (k - Self::ONE) * n.x * n.y;
        let nx_nz = (k - Self::ONE) * n.x * n.z;
        let ny_nz = (k - Self::ONE) * n.y * n.z;

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
        let two = Self::ONE + Self::ONE;

        Mat3 {
            mat: [
                [
                    Self::ONE - two * n.x.powi(2),
                    -two * n.x * n.y,
                    -two * n.x * n.z,
                ],
                [
                    -two * n.x * n.y,
                    Self::ONE - two * n.y.powi(2),
                    -two * n.x * n.z,
                ],
                [
                    -two * n.x * n.z,
                    -two * n.y * n.z,
                    Self::ONE - two * n.z.powi(2),
                ],
            ],
        }
    }

    #[inline]
    pub fn shearing_xy(s: T, t: T) -> Self {
        Mat3 {
            mat: [
                [Self::ONE, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ONE, Self::ZERO],
                [s, t, Self::ONE],
            ],
        }
    }

    #[inline]
    pub fn shearing_xz(s: T, t: T) -> Self {
        Mat3 {
            mat: [
                [Self::ONE, Self::ZERO, Self::ZERO],
                [s, Self::ONE, t],
                [Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }

    #[inline]
    pub fn shearing_yz(s: T, t: T) -> Self {
        Mat3 {
            mat: [
                [Self::ONE, s, t],
                [Self::ZERO, Self::ONE, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ONE],
            ],
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
        let sign = if (i + j) % 2 == 0 {
            Self::ONE
        } else {
            -Self::ONE
        };

        sign * self.minor(i, j)
    }

    #[inline]
    pub fn determinant(&self) -> T {
        let mut ret: T = Self::ZERO;

        //hardcoded column value since the result is the same in any other
        for i in 0..3 {
            ret += self[0][i] * self.cofactor(0, i);
        }
        ret
    }

    pub fn inverse(&self) -> Option<Mat3<T>> {
        let determinant = self.determinant();

        if determinant == Self::ZERO {
            return None;
        }

        let div = Self::ONE / determinant;
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

impl<T: FloatScalar> Default for Mat3<T> {
    fn default() -> Self {
        Mat3 {
            mat: [
                [Self::ONE, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ONE, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }
}
impl From<Quaternion> for Mat3<f32> {
    fn from(quat: Quaternion) -> Self {
        let x = quat.v.x;
        let y = quat.v.y;
        let z = quat.v.z;
        let w = quat.w;

        Mat3 {
            mat: [
                [
                    1.0 - 2.0 * y.powi(2) - 2.0 * z.powi(2),
                    2.0 * x * y + 2.0 * w * z,
                    2.0 * x * z - 2.0 * w * y,
                ],
                [
                    2.0 * x * y - 2.0 * w * z,
                    1.0 - 2.0 * x.powi(2) - 2.0 * z.powi(2),
                    2.0 * y * z + 2.0 * w * x,
                ],
                [
                    2.0 * x * z + 2.0 * w * y,
                    2.0 * y * z - 2.0 * w * x,
                    1.0 - 2.0 * x.powi(2) - 2.0 * y.powi(2),
                ],
            ],
        }
    }
}

impl From<Euler> for Mat3<f32> {
    fn from(euler: Euler) -> Self {
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
