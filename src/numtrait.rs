use std::fmt;

use crate::vectors::Vec2;
use num_traits::{real::Real, Float};

pub trait RealScalar: Copy + Clone + Real + Default + fmt::Debug {}

impl<T> RealScalar for T where T: Copy + Clone + fmt::Debug {}

pub trait FloatScalar: RealScalar + Float {}

impl<T> FloatScalar for T where T: RealScalar + Float {}
