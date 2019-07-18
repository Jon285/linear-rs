use crate::matrix::Mat3;
use crate::vectors::Vec3;
use crate::vectors::Vec4;

use std::default::Default;

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
    pub fn zero() -> Self {
        Mat4 {
            mat: [
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ],
        }
    }

    #[inline]
    pub fn from_vec(f: Vec4, s: Vec4, t: Vec4, l: Vec4) -> Self {
        Mat4 {
            mat: [
                [f.x, f.y, f.z, f.w],
                [s.x, s.y, s.z, s.w],
                [t.x, t.y, t.z, t.w],
                [l.x, l.y, l.z, l.w],
            ],
        }
    }

    //==========================================TRANSFORMATIONS=====================================

    #[inline]
    pub fn rotation_x(ang: f32) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();

        Mat4 {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, cos, sin, 0.0],
                [0.0, -sin, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn rotation_y(ang: f32) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();

        Mat4 {
            mat: [
                [cos, 0.0, sin, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-sin, 0.0, cos, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn rotation_z(ang: f32) -> Self {
        let cos = ang.cos();
        let sin = ang.sin();

        Mat4 {
            mat: [
                [cos, sin, 0.0, 0.0],
                [-sin, cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn rotation(ang: f32, n: Vec3) -> Self {
        let mat3 = Mat3::rotation(ang, n);
        let mut ret = Mat4::default();

        for i in 0..3 {
            ret[i][..3].clone_from_slice(&mat3[i][..3]);
        }
        ret
    }

    #[inline]
    pub fn translation(n: Vec3) -> Self {
        Mat4 {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [n.x, n.y, n.z, 1.0],
            ],
        }
    }

    #[inline]
    pub fn scale(k: f32) -> Self {
        Mat4 {
            mat: [
                [k, 0.0, 0.0, 0.0],
                [0.0, k, 0.0, 0.0],
                [0.0, 0.0, k, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
    ///Constructs a scale Matrix towards the arbitrary direction of `n` by a factor of `k`
    #[inline]
    pub fn scale_arb(k: f32, n: Vec3) -> Self {
        let n = n.normalized();

        //pre calculating some of the members
        let scale = 1.0 + (k - 1.0);
        let nx_ny = (k - 1.0) * n.x * n.y;
        let nx_nz = (k - 1.0) * n.x * n.z;
        let ny_nz = (k - 1.0) * n.y * n.z;

        Mat4 {
            mat: [
                [scale * n.x.powi(2), nx_ny, nx_nz, 0.0],
                [nx_ny, scale * n.y.powi(2), ny_nz, 0.0],
                [nx_nz, ny_nz, scale * n.z.powi(2), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn reflection(n: Vec3) -> Self {
        let n = n.normalized();

        Mat4 {
            mat: [
                [
                    1.0 - 2.0 * n.x.powi(2),
                    -2.0 * n.x * n.y,
                    -2.0 * n.x * n.z,
                    0.0,
                ],
                [
                    -2.0 * n.x * n.y,
                    1.0 - 2.0 * n.y.powi(2),
                    -2.0 * n.x * n.z,
                    0.0,
                ],
                [
                    -2.0 * n.x * n.z,
                    -2.0 * n.y * n.z,
                    1.0 - 2.0 * n.z.powi(2),
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn shearing_xy(s: f32, t: f32) -> Self {
        Mat4 {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [s, t, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn shearing_xz(s: f32, t: f32) -> Self {
        Mat4 {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [s, 1.0, t, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn shearing_yz(s: f32, t: f32) -> Self {
        Mat4 {
            mat: [
                [1.0, s, t, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    #[inline]
    pub fn perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let x_scale = 1.0 / (aspect * (fov / 2.0).tan());
        let y_scale = 1.0 / (fov / 2.0).tan();

        Mat4 {
            mat: [
                [x_scale, 0.0, 0.0, 0.0],
                [0.0, y_scale, 0.0, 0.0],
                [0.0, 0.0, (-near - far) / (near - far), 1.0],
                [0.0, 0.0, (2.0 * far * near) / (near - far), 0.0],
            ],
        }
    }

    ///Contructs a perspective matrix equivalent to the `glFrustum` one
    #[inline]
    pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let vertical = (top + bottom) / (top - bottom);
        let horizontal = (right + left) / (right - left);
        let depth = -(far + near) / (far - near);

        Mat4 {
            mat: [
                [(2.0 * near) / (right - 1.0), 0.0, 0.0, 0.0],
                [0.0, (2.0 * near) / (top - bottom), 0.0, 0.0],
                [horizontal, vertical, depth, -1.0],
                [0.0, 0.0, (-2.0 * far * near) / (far - near), 0.0],
            ],
        }
    }

    #[inline]
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let x_trans = -((right + left) / (right - left));
        let y_trans = -((top + bottom) / (top - bottom));
        let z_trans = -((far + near) / (far - near));;

        Mat4 {
            mat: [
                [2.0 / (right - left), 0.0, 0.0, 0.0],
                [0.0, 2.0 / (top - bottom), 0.0, 0.0],
                [0.0, 0.0, -2.0 / (far - near), 0.0],
                [x_trans, y_trans, z_trans, 1.0],
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
    pub fn minor(&self, i: usize, j: usize) -> f32 {
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

    pub fn cofactor(&self, i: usize, j: usize) -> f32 {
        (-1 as i32).pow((i + j) as u32) as f32 * self.minor(i, j)
    }

    pub fn determinant(&self) -> f32 {
        let mut ret: f32 = 0.0;

        for i in 0..4 {
            ret += self[i][0] * self.cofactor(i, 0);
        }
        ret
    }

    pub fn inverse(&self) -> Option<Mat4> {
        let determinant = self.determinant();

        if determinant == 0.0 {
            return None;
        }

        let div = 1.0 / determinant;
        let mut temp = Self::default();

        for i in 0..4 {
            for j in 0..4 {
                temp[j][i] = self.cofactor(i, j);
            }
        }

        Some(temp * div)
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
