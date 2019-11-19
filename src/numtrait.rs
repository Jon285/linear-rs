use std::fmt;

use crate::vectors::Vec2;
use num_traits::{Float, Num, NumAssign, NumAssignOps, NumCast};

pub trait RealScalar:
    Copy + Clone + Num + NumAssign + NumCast + NumAssignOps + Default + fmt::Debug
{
}

impl<T> RealScalar for T where T: Copy + NumCast + Clone + fmt::Debug + Num + NumAssign + Default {}

pub trait FloatScalar: RealScalar + Float {}

impl<T> FloatScalar for T where T: RealScalar + Float {}
