use std::default::Default;

#[repr(C)]
#[derive(Debug)]
pub struct Mat4 {
    pub mat: [[f32; 4]; 4],
}

impl Mat4 {}

impl Default for Mat4 {
    fn default() -> Self {
        Mat4 {
            mat: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}
