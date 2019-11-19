use num_traits::identities;

use std::convert::From;
use std::default::Default;

use crate::euler::Euler;
use crate::matrix::Mat3;
use crate::quaternions::Quaternion;
use crate::vectors::Vec3;
use crate::vectors::Vec4;
use crate::FloatScalar;

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

impl<T: FloatScalar> Mat4<T> {
    const ZERO: T = identities::zero::<T>();
    const ONE: T = identities::one::<T>();

    #[inline]
    pub fn zero() -> Self {
        Mat4 {
            mat: [
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ZERO],
            ],
        }
    }

    //==========================================TRANSFORMATIONS=====================================

    #[inline]
    pub fn rotation_x(ang: T) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();

        Mat4 {
            mat: [
                [Self::ONE, Self::ZERO, Self::ZERO, Self::ZERO],
                [Self::ZERO, cos, sin, Self::ZERO],
                [Self::ZERO, -sin, cos, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }

    #[inline]
    pub fn rotation_y(ang: T) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();

        Mat4 {
            mat: [
                [cos, Self::ZERO, -sin, Self::ZERO],
                [Self::ZERO, Self::ONE, Self::ZERO, Self::ZERO],
                [sin, Self::ZERO, cos, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }

    #[inline]
    pub fn rotation_z(ang: T) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();

        Mat4 {
            mat: [
                [cos, sin, Self::ZERO, Self::ZERO],
                [-sin, cos, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ONE, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ONE],
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
        Mat4 {
            mat: [
                [Self::ONE, Self::ZERO, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ONE, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ONE, Self::ZERO],
                [n.x, n.y, n.z, Self::ONE],
            ],
        }
    }

    #[inline]
    pub fn scale(k: T) -> Self {
        Mat4 {
            mat: [
                [k, Self::ZERO, Self::ZERO, Self::ZERO],
                [Self::ZERO, k, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ZERO, k, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }
    ///Constructs a scale Matrix towards the arbitrary direction of `n` by a factor of `k`
    #[inline]
    pub fn scale_arb(k: T, n: Vec3<T>) -> Self {
        let n = n.normalized();

        //pre calculating some of the members
        let scale = Self::ONE + (k - Self::ONE);
        let nx_ny = (k - Self::ONE) * n.x * n.y;
        let nx_nz = (k - Self::ONE) * n.x * n.z;
        let ny_nz = (k - Self::ONE) * n.y * n.z;

        Mat4 {
            mat: [
                [scale * n.x.powi(2), nx_ny, nx_nz, Self::ZERO],
                [nx_ny, scale * n.y.powi(2), ny_nz, Self::ZERO],
                [nx_nz, ny_nz, scale * n.z.powi(2), Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }

    #[inline]
    pub fn reflection(n: Vec3<T>) -> Self {
        let n = n.normalized();
        let two = Self::ONE + Self::ONE;

        Mat4 {
            mat: [
                [
                    Self::ONE - two * n.x.powi(2),
                    -two * n.x * n.y,
                    -two * n.x * n.z,
                    Self::ZERO,
                ],
                [
                    -two * n.x * n.y,
                    Self::ONE - two * n.y.powi(2),
                    -two * n.x * n.z,
                    Self::ZERO,
                ],
                [
                    -two * n.x * n.z,
                    -two * n.y * n.z,
                    Self::ONE - two * n.z.powi(2),
                    Self::ZERO,
                ],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }

    #[inline]
    pub fn shearing_xy(s: T, t: T) -> Self {
        Mat4 {
            mat: [
                [Self::ONE, Self::ZERO, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ONE, Self::ZERO, Self::ZERO],
                [s, t, Self::ONE, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }

    #[inline]
    pub fn shearing_xz(s: T, t: T) -> Self {
        Mat4 {
            mat: [
                [Self::ONE, Self::ZERO, Self::ZERO, Self::ZERO],
                [s, Self::ONE, t, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ONE, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }

    #[inline]
    pub fn shearing_yz(s: T, t: T) -> Self {
        Mat4 {
            mat: [
                [Self::ONE, s, t, Self::ZERO],
                [Self::ZERO, Self::ONE, Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ONE, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }

    #[inline]
    pub fn perspective(fov: T, aspect: T, near: T, far: T) -> Self {
        let two = Self::ONE + Self::ONE;
        let x_scale = Self::ONE / (aspect * (fov / two).tan());
        let y_scale = Self::ONE / (fov / two).tan();

        Mat4 {
            mat: [
                [x_scale, Self::ZERO, Self::ZERO, Self::ZERO],
                [Self::ZERO, y_scale, Self::ZERO, Self::ZERO],
                [
                    Self::ZERO,
                    Self::ZERO,
                    (-near - far) / (near - far),
                    Self::ONE,
                ],
                [
                    Self::ZERO,
                    Self::ZERO,
                    (two * far * near) / (near - far),
                    Self::ZERO,
                ],
            ],
        }
    }

    ///Contructs a perspective matrix. Equivalent to `glFrustum`
    #[inline]
    pub fn frustum(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let vertical = (top + bottom) / (top - bottom);
        let horizontal = (right + left) / (right - left);
        let depth = -(far + near) / (far - near);
        let two = Self::ONE + Self::ONE;

        Mat4 {
            mat: [
                [
                    (two * near) / (right - Self::ONE),
                    Self::ZERO,
                    Self::ZERO,
                    Self::ZERO,
                ],
                [
                    Self::ZERO,
                    (two * near) / (top - bottom),
                    Self::ZERO,
                    Self::ZERO,
                ],
                [horizontal, vertical, depth, -Self::ONE],
                [
                    Self::ZERO,
                    Self::ZERO,
                    (-two * far * near) / (far - near),
                    Self::ZERO,
                ],
            ],
        }
    }

    ///Constructs an orthographic projection matrix
    #[inline]
    pub fn ortho(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let x_trans = -((right + left) / (right - left));
        let y_trans = -((top + bottom) / (top - bottom));
        let z_trans = -((far + near) / (far - near));
        let two = Self::ONE + Self::ONE;

        Mat4 {
            mat: [
                [two / (right - left), Self::ZERO, Self::ZERO, Self::ZERO],
                [Self::ZERO, two / (top - bottom), Self::ZERO, Self::ZERO],
                [Self::ZERO, Self::ZERO, -two / (far - near), Self::ZERO],
                [x_trans, y_trans, z_trans, Self::ONE],
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

        Mat4 {
            mat: [
                [x.x, y.x, z.x, Self::ZERO],
                [x.y, y.y, z.y, Self::ZERO],
                [x.z, y.z, z.z, Self::ZERO],
                [-x.dot(eye), -y.dot(eye), -z.dot(eye), Self::ONE],
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
        let mut ret: T = Self::ZERO;

        for i in 0..4 {
            ret += self[i][0] * self.cofactor(i, 0);
        }
        ret
    }

    pub fn inverse(&self) -> Option<Mat4<T>> {
        let determinant = self.determinant();

        if determinant == Self::ZERO {
            return None;
        }

        let div = Self::ONE / determinant;
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

impl<T: FloatScalar> Default for Mat4<T> {
    fn default() -> Self {
        Mat4::new(
            Self::ONE,
            Self::ZERO,
            Self::ZERO,
            Self::ZERO,
            Self::ZERO,
            Self::ONE,
            Self::ZERO,
            Self::ZERO,
            Self::ZERO,
            Self::ZERO,
            Self::ONE,
            Self::ZERO,
            Self::ZERO,
            Self::ZERO,
            Self::ZERO,
            Self::ONE,
        )
    }
}

impl From<Quaternion> for Mat4<f32> {
    fn from(quat: Quaternion) -> Self {
        let x = quat.v.x;
        let y = quat.v.y;
        let z = quat.v.z;
        let w = quat.w;

        Mat4 {
            mat: [
                [
                    Self::ONE - 2.0 * y.powi(2) - 2.0 * z.powi(2),
                    2.0 * x * y + 2.0 * w * z,
                    2.0 * x * z - 2.0 * w * y,
                    Self::ZERO,
                ],
                [
                    2.0 * x * y - 2.0 * w * z,
                    Self::ONE - 2.0 * x.powi(2) - 2.0 * z.powi(2),
                    2.0 * y * z + 2.0 * w * x,
                    Self::ZERO,
                ],
                [
                    2.0 * x * z + 2.0 * w * y,
                    2.0 * y * z - 2.0 * w * x,
                    Self::ONE - 2.0 * x.powi(2) - 2.0 * y.powi(2),
                    Self::ZERO,
                ],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ONE],
            ],
        }
    }
}

impl From<Euler> for Mat4<f32> {
    fn from(euler: Euler) -> Self {
        let cy = euler.yaw.cos();
        let cp = euler.pitch.cos();
        let cr = euler.row.cos();
        let sy = euler.yaw.cos();
        let sp = euler.pitch.cos();
        let sr = euler.row.sin();

        Mat4 {
            mat: [
                [
                    cy * cr + sy * sp * sr,
                    sr * cp,
                    -sy * cr + cy * sp * sr,
                    Self::ZERO,
                ],
                [
                    -cy * sr + sy * sp * cr,
                    cr * cp,
                    sr * sy + cy * sp * cr,
                    Self::ZERO,
                ],
                [sy * cp, -sp, cy * cp, Self::ZERO],
                [Self::ZERO, Self::ZERO, Self::ZERO, Self::ONE],
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
