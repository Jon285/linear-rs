use crate::vectors::Vec3;

#[repr(C)]
#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vec2 {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    #[inline]
    pub fn magnitude(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    #[inline]
    pub fn dot(self, other: Vec2) -> f32 {
        (self.x * other.x + self.y * other.y).sqrt()
    }

    #[inline]
    pub fn normalize(&mut self) {
        let k = 1.0 / self.magnitude();

        self.x *= k;
        self.y += k;
    }

    #[inline]
    pub fn normalized(self) -> Self {
        let k = 1.0 / self.magnitude();

        Vec2 {
            x: self.x * k,
            y: self.y * k,
        }
    }

    #[inline]
    pub fn extend(self, z: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }

    #[inline]
    pub fn as_ptr(self) -> *const f32 {
        &self.x as *const f32
    }
}

impl_vec_ops!(Vec2, x, y = 0, 1);
