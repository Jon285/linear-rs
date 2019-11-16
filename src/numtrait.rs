use std::fmt;

use crate::vectors::Vec2;
use num_traits::{Float, Num, NumOps};

pub trait NumScalar: Copy + Clone + Num + Default + fmt::Debug {}

impl<T> NumScalar for T where T: Copy + Clone + Num + fmt::Debug {}

pub trait FloatScalar: NumScalar + Float {}

impl<T> FloatScalar for T where T: NumScalar + Float {}
