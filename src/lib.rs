#![crate_type = "lib"]

mod matrix;
mod vectors;

#[allow(unused_imports)]
use matrix::Mat3;
#[allow(unused_imports)]
use vectors::Vec3;
#[allow(unused_imports)]
use vectors::Vec4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_default() {
        let testm = Mat3::default();
        let compare = Mat3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);

        assert_eq!(testm, compare);
    }

    #[test]
    fn matrix_trans() {
        let mut original = Mat3::new(1.0, 2.0, 1.0, 3.0, 4.0, 5.0, 0.0, 1.0, 2.0);
        let transpost = original.transpost();
        original.transpose();

        assert_eq!(original, transpost);
    }

    #[test]
    fn matrix_array() {
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
    fn matrix_ident_mul() {
        let identity = Mat3::default();
        let other = Mat3::new(5.0, 3.2, 5.1, 2.0, 1.0, 0.0, 3.2, 4.0, 1.1);

        assert_eq!(other, other * identity);
    }
}
