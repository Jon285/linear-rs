use crate::vectors::Vec3;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[allow(dead_code)]
impl Vec4 {
    #[inline]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4 { x, y, z, w }
    }

    #[inline]
    pub fn magnitude(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    #[inline]
    pub fn dot(self, other: Vec4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let s = 1.0 / self.magnitude();

        Vec4 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,
        }
    }

    #[inline]
    pub fn normalize(&mut self) {
        let s = 1.0 / self.magnitude();

        self.x *= s;
        self.y *= s;
        self.z *= s;
        self.w *= s;
    }

    #[inline]
    pub fn truncate(self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl_vec_ops!(Vec4, x, y, z, w = 0, 1, 2, 3);
