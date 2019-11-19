macro_rules! impl_vec_ops {
    ($VecN:ident, $($field:ident),+ = $($dimensions:pat),+) => {
        use std::ops::*;
        use crate::RealScalar;

            impl<T: RealScalar> Add<$VecN<T>> for $VecN<T> {
                type Output = Self;

                fn add(self, other: $VecN<T>) -> Self {
                    $VecN {
                        $(
                            $field: self.$field + other.$field,
                        )+
                    }
                }
            }

            impl<T: RealScalar> AddAssign<$VecN<T>> for $VecN<T> {
                fn add_assign(&mut self, other: $VecN<T>) {
                    *self = $VecN {
                        $(
                            $field: self.$field + other.$field,
                        )+
                    }
                }
            }

            impl<T: RealScalar> Sub<$VecN<T>> for $VecN<T> {
                type Output = Self;

                fn sub(self, other: $VecN<T>) -> Self {
                    $VecN {
                        $(
                            $field: self.$field - other.$field,
                        )+
                    }
                }
            }

            impl<T: RealScalar> SubAssign<$VecN<T>> for $VecN<T> {
                fn sub_assign(&mut self, other: $VecN<T>) {
                    *self = $VecN {
                        $(
                            $field: self.$field - other.$field,
                        )+
                    }
                }
            }

            impl<T: RealScalar> Mul<$VecN<T>> for $VecN<T> {
                type Output = Self;

                fn mul(self, other: $VecN<T>) -> Self {
                    $VecN {
                        $(
                            $field: self.$field * other.$field,
                        )+
                    }
                }
            }

            impl<T: RealScalar> MulAssign<$VecN<T>> for $VecN<T> {
                fn mul_assign(&mut self, other: $VecN<T>) {
                    *self = $VecN {
                        $(
                            $field: self.$field * other.$field,
                        )+
                    }
                }
            }

            impl<T: RealScalar> Mul<T> for $VecN<T> {
                type Output = Self;

                fn mul(self, other: T) -> Self {
                    $VecN {
                        $(
                            $field: self.$field * other,
                        )+
                    }
                }
            }

           impl<T: RealScalar> MulAssign<T> for $VecN<T> {
                fn mul_assign(&mut self, other: T) {
                    *self = $VecN {
                        $(
                            $field: self.$field * other,
                        )+
                    }
                }
            }

            impl<T: RealScalar> Div<T> for $VecN<T> {
                type Output = Self;

                fn div(self, other: T) -> Self {
                    $VecN {
                        $(
                            $field: self.$field / other,
                        )+
                    }
                }
            }

            impl<T: RealScalar> DivAssign<T> for $VecN<T> {
                fn div_assign(&mut self, other: T) {
                    *self = $VecN {
                        $(
                            $field: self.$field / other,
                        )+
                    }
                }
            }

        impl Mul<$VecN<f32>> for f32
        {
                type Output = $VecN<f32>;

                fn mul(self, other: $VecN<f32>) -> $VecN<f32> {
                    $VecN {
                        $(
                            $field: other.$field * self,
                        )+
                    }
                }
            }

            impl Neg for $VecN<f32> {
                type Output = $VecN<f32>;

                fn neg(self) -> Self {
                    $VecN {
                        $(
                            $field: -self.$field,
                        )+
                    }
                }
            }

            impl<T: std::fmt::Debug> Index<usize> for $VecN<T> {
                type Output = T;

                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        $($dimensions => &self.$field,)+
                        _ => panic!("PANIC. Out of bonds access on Vector: {:?}\nWith index: {}", self, index),
                    }
                }
            }

            impl<T: std::fmt::Debug> IndexMut<usize> for $VecN<T> {
                fn index_mut(&mut self, index: usize) -> &mut T {
                    match index {
                        $($dimensions => &mut self.$field,)+
                        _ => panic!("PANIC. Out of bonds access on Vector: {:?}\nWith index: {}", self, index),
                    }
                }
            }
    };
}
