use num_traits::identities;

use std::convert::From;
use std::default::Default;

use crate::euler::Euler;
use crate::matrix::Mat3;
use crate::quaternions::Quaternion;
use crate::vectors::Vec3;
use crate::vectors::Vec4;
use crate::{FloatScalar, RealScalar};

///A column major 4x4 matrix
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Mat4<T> {
    pub(crate) mat: [[T; 4]; 4],
}

#[allow(dead_code)]
impl<T> Mat4<T> {
    #[inline]
    pub const fn new(
        s0e0: T,
        s0e1: T,
        s0e2: T,
        s0e3: T,
        s1e0: T,
        s1e1: T,
        s1e2: T,
        s1e3: T,
        s2e0: T,
        s2e1: T,
        s2e2: T,
        s2e3: T,
        s3e0: T,
        s3e1: T,
        s3e2: T,
        s3e3: T,
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
}

impl<T: RealScalar> Mat4<T> {
    #[inline]
    pub fn zero() -> Self {
        let zero = identities::zero::<T>();
        Mat4 {
            mat: [
                [zero, zero, zero, zero],
                [zero, zero, zero, zero],
                [zero, zero, zero, zero],
                [zero, zero, zero, zero],
            ],
        }
    }
}

impl<T: FloatScalar> Mat4<T> {
    //==========================================TRANSFORMATIONS=====================================

    #[inline]
    pub fn rotation_x(ang: T) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();

        Mat4 {
            mat: [
                [one, zero, zero, zero],
                [zero, cos, sin, zero],
                [zero, -sin, cos, zero],
                [zero, zero, zero, one],
            ],
        }
    }

    #[inline]
    pub fn rotation_y(ang: T) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();

        Mat4 {
            mat: [
                [cos, zero, -sin, zero],
                [zero, one, zero, zero],
                [sin, zero, cos, zero],
                [zero, zero, zero, one],
            ],
        }
    }

    #[inline]
    pub fn rotation_z(ang: T) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();

        Mat4 {
            mat: [
                [cos, sin, zero, zero],
                [-sin, cos, zero, zero],
                [zero, zero, one, zero],
                [zero, zero, zero, one],
            ],
        }
    }

    #[inline]
    pub fn rotation(ang: T, n: Vec3<T>) -> Self {
        let mat3 = Mat3::rotation(ang, n);
        let mut ret = Mat4::default();

        for i in 0..3 {
            ret[i][..3].clone_from_slice(&mat3[i][..3]);
        }
        ret
    }

    #[inline]
    pub fn translation(n: Vec3<T>) -> Self {
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        Mat4 {
            mat: [
                [one, zero, zero, zero],
                [zero, one, zero, zero],
                [zero, zero, one, zero],
                [n.x, n.y, n.z, one],
            ],
        }
    }

    #[inline]
    pub fn scale(k: T) -> Self {
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        Mat4 {
            mat: [
                [k, zero, zero, zero],
                [zero, k, zero, zero],
                [zero, zero, k, zero],
                [zero, zero, zero, one],
            ],
        }
    }
    ///Constructs a scale Matrix towards the arbitrary direction of `n` by a factor of `k`
    #[inline]
    pub fn scale_arb(k: T, n: Vec3<T>) -> Self {
        let n = n.normalized();

        //pre calculating some of the members
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        let scale = one + (k - one);
        let nx_ny = (k - one) * n.x * n.y;
        let nx_nz = (k - one) * n.x * n.z;
        let ny_nz = (k - one) * n.y * n.z;

        Mat4 {
            mat: [
                [scale * n.x.powi(2), nx_ny, nx_nz, zero],
                [nx_ny, scale * n.y.powi(2), ny_nz, zero],
                [nx_nz, ny_nz, scale * n.z.powi(2), zero],
                [zero, zero, zero, one],
            ],
        }
    }

    #[inline]
    pub fn reflection(n: Vec3<T>) -> Self {
        let n = n.normalized();
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        let two = one + one;

        Mat4 {
            mat: [
                [
                    one - two * n.x.powi(2),
                    -two * n.x * n.y,
                    -two * n.x * n.z,
                    zero,
                ],
                [
                    -two * n.x * n.y,
                    one - two * n.y.powi(2),
                    -two * n.x * n.z,
                    zero,
                ],
                [
                    -two * n.x * n.z,
                    -two * n.y * n.z,
                    one - two * n.z.powi(2),
                    zero,
                ],
                [zero, zero, zero, one],
            ],
        }
    }

    #[inline]
    pub fn shearing_xy(s: T, t: T) -> Self {
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        Mat4 {
            mat: [
                [one, zero, zero, zero],
                [zero, one, zero, zero],
                [s, t, one, zero],
                [zero, zero, zero, one],
            ],
        }
    }

    #[inline]
    pub fn shearing_xz(s: T, t: T) -> Self {
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        Mat4 {
            mat: [
                [one, zero, zero, zero],
                [s, one, t, zero],
                [zero, zero, one, zero],
                [zero, zero, zero, one],
            ],
        }
    }

    #[inline]
    pub fn shearing_yz(s: T, t: T) -> Self {
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        Mat4 {
            mat: [
                [one, s, t, zero],
                [zero, one, zero, zero],
                [zero, zero, one, zero],
                [zero, zero, zero, one],
            ],
        }
    }

    #[inline]
    pub fn perspective(fov: T, aspect: T, near: T, far: T) -> Self {
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        let two = one + one;
        let x_scale = one / (aspect * (fov / two).tan());
        let y_scale = one / (fov / two).tan();

        Mat4 {
            mat: [
                [x_scale, zero, zero, zero],
                [zero, y_scale, zero, zero],
                [zero, zero, (-near - far) / (near - far), one],
                [zero, zero, (two * far * near) / (near - far), zero],
            ],
        }
    }

    ///Contructs a perspective matrix. Equivalent to `glFrustum`
    #[inline]
    pub fn frustum(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let vertical = (top + bottom) / (top - bottom);
        let horizontal = (right + left) / (right - left);
        let depth = -(far + near) / (far - near);
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        let two = one + one;

        Mat4 {
            mat: [
                [(two * near) / (right - one), zero, zero, zero],
                [zero, (two * near) / (top - bottom), zero, zero],
                [horizontal, vertical, depth, -one],
                [zero, zero, (-two * far * near) / (far - near), zero],
            ],
        }
    }

    ///Constructs an orthographic projection matrix
    #[inline]
    pub fn ortho(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let x_trans = -((right + left) / (right - left));
        let y_trans = -((top + bottom) / (top - bottom));
        let z_trans = -((far + near) / (far - near));
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        let two = one + one;

        Mat4 {
            mat: [
                [two / (right - left), zero, zero, zero],
                [zero, two / (top - bottom), zero, zero],
                [zero, zero, -two / (far - near), zero],
                [x_trans, y_trans, z_trans, one],
            ],
        }
    }

    ///Constructs a new view matrix based on the `eye` (aka camera position),
    ///the `direction` to look and and the camera's `up` direction.
    #[inline]
    pub fn look_at(eye: Vec3<T>, direction: Vec3<T>, up: Vec3<T>) -> Self {
        let z = (eye - direction).normalized();
        let x = up.cross(z).normalized();
        let y = z.cross(x).normalized();
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();

        Mat4 {
            mat: [
                [x.x, y.x, z.x, zero],
                [x.y, y.y, z.y, zero],
                [x.z, y.z, z.z, zero],
                [-x.dot(eye), -y.dot(eye), -z.dot(eye), one],
            ],
        }
    }

    //================================================================================================

    #[inline]
    pub fn transpose(&mut self) {
        let mut temp = Mat4::default();

        for i in 0..=3 {
            for j in 0..=3 {
                temp[i][j] = self[j][i];
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

    //not the best algorithm, but works fine for the purpose of this lib
    pub fn minor(&self, i: usize, j: usize) -> T {
        if i > 3 || j > 3 {
            panic!("out of bonds matrix access");
        }

        let mut matrix_3 = Mat3::default();
        let mut col = 0;
        let mut row = 0;

        for a in 0..4 {
            for b in 0..4 {
                //are we in the excluded row or column?
                if a != i && b != j {
                    matrix_3[col][row] = self[a][b];
                    row += 1;

                    //column is filled, change to the next and reset the row
                    if row == 3 {
                        row = 0;
                        col += 1;
                    }
                }
            }
        }
        matrix_3.determinant()
    }

    pub fn cofactor(&self, i: usize, j: usize) -> T {
        T::from::<i32>((-1 as i32).pow((i + j) as u32)).unwrap() * self.minor(i, j)
    }

    pub fn determinant(&self) -> T {
        let mut ret = identities::zero::<T>();

        for i in 0..4 {
            ret += self[i][0] * self.cofactor(i, 0);
        }
        ret
    }

    pub fn inverse(&self) -> Option<Mat4<T>> {
        let determinant = self.determinant();
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();

        if determinant == zero {
            return None;
        }

        let div = one / determinant;
        let mut temp = Self::default();

        for i in 0..4 {
            for j in 0..4 {
                temp[j][i] = self.cofactor(i, j);
            }
        }

        Some(temp * div)
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

impl_mat_ops!(Mat4, mat, 4, [T; 4]);
impl_mat_ops!(Mat4, Vec4, 4);

impl<T: RealScalar> Default for Mat4<T> {
    fn default() -> Self {
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        Mat4::new(
            one, zero, zero, zero, zero, one, zero, zero, zero, zero, one, zero, zero, zero, zero,
            one,
        )
    }
}

impl<T: FloatScalar> From<Quaternion<T>> for Mat4<T> {
    fn from(quat: Quaternion<T>) -> Self {
        let x = quat.v.x;
        let y = quat.v.y;
        let z = quat.v.z;
        let w = quat.w;
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();
        let two = one + one;

        Mat4 {
            mat: [
                [
                    one - two * y.powi(2) - two * z.powi(2),
                    two * x * y + two * w * z,
                    two * x * z - two * w * y,
                    zero,
                ],
                [
                    two * x * y - two * w * z,
                    one - two * x.powi(2) - two * z.powi(2),
                    two * y * z + two * w * x,
                    zero,
                ],
                [
                    two * x * z + two * w * y,
                    two * y * z - two * w * x,
                    one - two * x.powi(2) - two * y.powi(2),
                    zero,
                ],
                [zero, zero, zero, one],
            ],
        }
    }
}

impl<T: FloatScalar> From<Euler<T>> for Mat4<T> {
    fn from(euler: Euler<T>) -> Self {
        let cy = euler.yaw.cos();
        let cp = euler.pitch.cos();
        let cr = euler.row.cos();
        let sy = euler.yaw.cos();
        let sp = euler.pitch.cos();
        let sr = euler.row.sin();
        let zero = identities::zero::<T>();
        let one = identities::one::<T>();

        Mat4 {
            mat: [
                [
                    cy * cr + sy * sp * sr,
                    sr * cp,
                    -sy * cr + cy * sp * sr,
                    zero,
                ],
                [
                    -cy * sr + sy * sp * cr,
                    cr * cp,
                    sr * sy + cy * sp * cr,
                    zero,
                ],
                [sy * cp, -sp, cy * cp, zero],
                [zero, zero, zero, one],
            ],
        }
    }
}

impl<T> From<[[T; 4]; 4]> for Mat4<T> {
    fn from(array: [[T; 4]; 4]) -> Self {
        Mat4 { mat: array }
    }
}

impl<T> From<(Vec4<T>, Vec4<T>, Vec4<T>, Vec4<T>)> for Mat4<T> {
    fn from(tuple: (Vec4<T>, Vec4<T>, Vec4<T>, Vec4<T>)) -> Self {
        Mat4 {
            mat: [
                [tuple.0.x, tuple.0.y, tuple.0.z, tuple.0.w],
                [tuple.1.x, tuple.1.y, tuple.1.z, tuple.1.w],
                [tuple.2.x, tuple.2.y, tuple.2.z, tuple.2.w],
                [tuple.3.x, tuple.3.y, tuple.3.z, tuple.3.w],
            ],
        }
    }
}
