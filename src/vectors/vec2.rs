use std::ops::*;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[allow(dead_code)]
impl Vec2 {
    #[inline]
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x: x, y: y }
    }

    #[inline]
    pub fn len(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    #[inline]
    pub fn dot(&self, other: &Vec2) -> f32 {
        (self.x * other.x + self.y * other.y).sqrt()
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        let k = 1.0 / self.len();

        Vec2 {
            x: self.x * k,
            y: self.y * k,
        }
    }
}

impl_vec_ops!(Vec2, x, y);

impl Index<usize> for Vec2 {
    type Output = f32;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!(
                "PANIC. Out of bonds index access on Vector: {:?}\nWith index: {}\n",
                self, index
            ),
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!(
                "PANIC. Out of bons access on Vector: {:?}\nWith index: {}\n",
                self, index
            ),
        }
    }
}
