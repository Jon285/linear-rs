#![crate_type = "lib"]
#![allow(unused_imports)]

extern crate num_traits;

mod euler;
mod matrix;
mod quaternions;
mod vectors;

pub use euler::Euler;
pub use matrix::Mat2;
pub use matrix::Mat3;
pub use matrix::Mat4;
pub use quaternions::Quaternion;
pub use vectors::Vec2;
pub use vectors::Vec3;
pub use vectors::Vec4;

#[macro_export]
macro_rules! abs_diff_eq {
    ($lhs:expr, $rhs:expr) => {
        ($lhs - $rhs) < std::f32::EPSILON
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_index() {
        let vec2 = Vec2::new(1.0, 2.0);
        let vec3 = Vec3::new(1.0, 2.0, 3.0);
        let vec4 = Vec4::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(vec2[0], vec2.x);
        assert_eq!(vec2[1], vec2.y);

        assert_eq!(vec3[0], vec3.x);
        assert_eq!(vec3[1], vec3.y);
        assert_eq!(vec3[2], vec3.z);

        assert_eq!(vec4[0], vec4.x);
        assert_eq!(vec4[1], vec4.y);
        assert_eq!(vec4[2], vec4.z);
        assert_eq!(vec4[3], vec4.w);
    }

    #[test]
    fn vec2_sub() {
        let vec = Vec2::new(1.0, 1.0);
        let other = Vec2::new(1.0, 1.0);
        let res = Vec2::new(0.0, 0.0);
        let sub = vec - other;

        assert_eq!(res.x, sub.x);
        assert_eq!(res.x, sub.x);
    }

    #[test]
    fn vec2_sum() {
        let vec = Vec2::new(1.0, 1.0);
        let other = Vec2::new(1.0, 1.0);
        let res = Vec2::new(2.0, 2.0);
        let sum = vec + other;

        assert_eq!(res.x, sum.x);
        assert_eq!(res.y, sum.y);
    }

    #[test]
    fn mat_vec_mul() {
        let mat = Mat3::default();
        let vec = Vec3::new(3.0, 3.0, 3.0);

        assert_eq!(vec, mat * vec);
    }

    #[test]
    fn mat_ops_sum() {
        let m2 = Mat2::new(1.0, 1.0, 1.0, 1.0);
        let other_m2 = m2;
        let m2_res = Mat2::new(2.0, 2.0, 2.0, 2.0);

        let m3 = Mat3::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
        let other_m3 = m3;
        let m3_res = Mat3::new(2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0);

        let m4 = Mat4::new(
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        );
        let other_m4 = m4;
        let m4_res = Mat4::new(
            2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0,
        );

        assert_eq!(m2_res, m2 + other_m2);
        assert_eq!(m3_res, m3 + other_m3);
        assert_eq!(m4_res, m4 + other_m4);
    }

    #[test]
    fn mat2_ident_mul() {
        let ident = Mat2::default();
        let mat = Mat2::new(2.0, 3.4, 5.0, 2.1);

        assert_eq!(mat, mat * ident);
    }

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

    //#[test]
    //fn mat3_array() {
    //let arr: [[f32; 3]; 3] = [[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]];
    //let mat = Mat3::from_array(&arr);
    //let mat_comp = Mat3::new(1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
    //
    //assert_eq!(mat, mat_comp);
    //}

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

        assert_eq!(other, other * identity);
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

    //#[test]
    //fn mat4_array() {
    //let arr: [[f32; 4]; 4] = [
    //[1.0, 1.0, 1.0, 1.0],
    //[1.0, 1.0, 1.0, 1.0],
    //[1.0, 1.0, 1.0, 1.0],
    //[1.0, 1.0, 1.0, 1.0],
    //];
    //
    //let mat = Mat4::from_array(&arr);
    //let mat_cmp = Mat4::new(
    //1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    //);
    //
    //assert_eq!(mat, mat_cmp);
    //}

    #[test]
    fn ident_inverse() {
        let mat = Mat3::default();

        assert_eq!(mat, mat.inverse().unwrap());
    }

    #[test]
    fn mat3_minor() {
        let mat = Mat3::new(1.0, 3.0, -1.0, 4.0, 0.0, 9.0, 7.0, 5.0, 11.0);

        assert_eq!(13.0, mat.minor(2, 1));
    }

    #[test]
    fn mat3_cof() {
        let mat = Mat3::new(-4.0, -3.0, 3.0, 0.0, 2.0, -2.0, 1.0, 4.0, -1.0);

        assert_eq!(6.0, mat.cofactor(0, 0));
    }

    #[test]
    fn mat3_rotation() {
        let mat = Mat3::rotation_z(-180.0_f32.to_radians());
        let vec = Vec3::new(0.0, 1.0, 0.0);
        let res = Vec3::new(0.0, -1.0, 0.0);
        let mul = mat * vec;

        assert!(abs_diff_eq!(mul.x, res.x));
        assert!(abs_diff_eq!(mul.y, res.y));
        assert!(abs_diff_eq!(mul.z, res.z));
    }

    #[test]
    fn quaternion_rotation() {
        let quat = Quaternion::new(-90.0_f32.to_radians(), Vec3::new(0.0, 0.0, 1.0));
        let mat = Mat4::from(quat);
        let vec = Vec4::new(1.0, 0.0, 0.0, 1.0);
        let res = Vec4::new(0.0, -1.0, 0.0, 1.0);
        let mul = mat * vec;

        assert!(abs_diff_eq!(mul.x, res.x));
        assert!(abs_diff_eq!(mul.y, res.y));
        assert!(abs_diff_eq!(mul.z, res.z));
        assert!(abs_diff_eq!(mul.w, res.w));
    }
}
