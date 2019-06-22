#![crate_type = "lib"]

mod matrix;
mod vectors;

#[allow(unused_imports)]
pub use matrix::Mat3;
#[allow(unused_imports)]
pub use matrix::Mat4;
#[allow(unused_imports)]
pub use vectors::Vec3;
#[allow(unused_imports)]
pub use vectors::Vec4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mat3_default() {
        let testm = Mat3::default();
        let compare = Mat3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);

        assert_eq!(testm, compare);
    }

    #[test]
    fn mat3_trans() {
        let mut original = Mat3::new(1.0, 2.0, 1.0, 3.0, 4.0, 5.0, 0.0, 1.0, 2.0);
        let transpost = original.transpost();
        original.transpose();

        assert_eq!(original, transpost);
    }

    #[test]
    fn mat3_array() {
        let arr: [[f32; 3]; 3] = [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]];
        let mat = Mat3::from_array(&arr);
        let mat_comp = Mat3::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);

        assert_eq!(mat, mat_comp);
    }

    #[test]
    fn mat3_index() {
        let mat = Mat3::new(0.0, 0.1, 0.2, 1.0, 1.1, 1.2, 2.0, 2.1, 2.2);

        assert_eq!(mat[0][0], 0.0);
        assert_eq!(mat[0][1], 0.1);
        assert_eq!(mat[0][2], 0.2);
        assert_eq!(mat[1][0], 1.0);
        assert_eq!(mat[1][1], 1.1);
        assert_eq!(mat[1][2], 1.2);
        assert_eq!(mat[2][0], 2.0);
        assert_eq!(mat[2][1], 2.1);
        assert_eq!(mat[2][2], 2.2);
    }

    #[test]
    #[should_panic]
    fn mat3_index_fail() {
        let mat = Mat3::default();

        mat[5][2];
    }

    #[test]
    fn mat3_ident_mul() {
        let identity = Mat3::default();
        let other = Mat3::new(5.0, 3.2, 5.1, 2.0, 1.0, 0.0, 3.2, 4.0, 1.1);

        assert_eq!(other, other * identity);
    }

    #[test]
    fn mat4_index() {
        let mat = Mat4::new(
            0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
        );

        assert_eq!(mat[0][0], 0.0);
        assert_eq!(mat[0][1], 1.0);
        assert_eq!(mat[0][2], 2.0);
        assert_eq!(mat[0][3], 3.0);
        assert_eq!(mat[1][0], 4.0);
        assert_eq!(mat[1][1], 5.0);
        assert_eq!(mat[1][2], 6.0);
        assert_eq!(mat[1][3], 7.0);
        assert_eq!(mat[2][0], 8.0);
        assert_eq!(mat[2][1], 9.0);
        assert_eq!(mat[2][2], 10.0);
        assert_eq!(mat[2][3], 11.0);
        assert_eq!(mat[3][0], 12.0);
        assert_eq!(mat[3][1], 13.0);
        assert_eq!(mat[3][2], 14.0);
        assert_eq!(mat[3][3], 15.0);
    }

    #[test]
    fn mat4_ident_mul() {
        let identity = Mat4::default();
        let other = Mat4::new(
            1.0, 3.0, 4.2, 0.3, 0.3, 7.3, 0.2, 5.1, 7.0, 10.2, 1.2, 5.6, 1.0, 3.1, 2.0, 7.0,
        );

        assert_eq!(other, other);
    }

    #[test]
    fn mat4_trans_cmp() {
        let mut mat = Mat4::default();
        let other = mat.transpost();
        mat.transpose();

        assert_eq!(mat, other);
    }

    #[test]
    fn mat4_ident_trans() {
        let mat = Mat4::default();

        assert_eq!(mat, mat.transpost());
    }

    #[test]
    fn mat4_array() {
        let arr: [[f32; 4]; 4] = [
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0, 1.0],
        ];

        let mat = Mat4::from_array(&arr);
        let mat_cmp = Mat4::new(
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        );

        assert_eq!(mat, mat_cmp);
    }
}
