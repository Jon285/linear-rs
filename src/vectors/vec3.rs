use std::ops;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new(xi: f32, yi: f32, zi: f32) -> Self {
        Vec3 {
            x: xi,
            y: yi,
            z: zi,
        }
    }

    #[inline]
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    #[inline]
    pub fn squared_len(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn normalize(&mut self) {
        let k = 1.0 / self.len();
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let k = 1.0 / self.len();
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    #[inline]
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    #[inline]
    pub fn cross(&self, b: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: -(self.x * b.z - self.z * b.x),
            z: self.x * b.y - self.y * b.x,
        }
    }

    #[inline]
    pub fn distance_to(&self, other: &Vec3) -> f32 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2))
            .sqrt()
    }

    #[inline]
    pub fn vector_to(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }
}

impl_vec_ops!(Vec3, x, y, z = 0, 1, 2);
