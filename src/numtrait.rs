use std::fmt;

use crate::vectors::Vec2;
use num_traits::{Float, Num, NumAssign, NumAssignOps, NumCast};
use std::ops;

pub trait RealScalar:
    Copy
    + Clone
    + Num
    + NumAssign
    + NumCast
    + NumAssignOps
    + Default
    + fmt::Debug
    + ops::Neg<Output = Self>
{
}

impl<T> RealScalar for T where
    T: Copy + NumCast + Clone + fmt::Debug + Num + NumAssign + Default + ops::Neg<Output = Self>
{
}

pub trait FloatScalar: RealScalar + Float {}

impl<T> FloatScalar for T where T: RealScalar + Float {}
