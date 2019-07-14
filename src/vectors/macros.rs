macro_rules! impl_vec_ops {
    ($VecN:ident, $($field:ident),+ = $($dimensions:pat),+) => {
            use std::ops::*;

            impl Add<$VecN> for $VecN {
                type Output = Self;

                fn add(self, other: $VecN) -> Self {
                    $VecN {
                        $(
                            $field: self.$field + other.$field,
                        )+
                    }
                }
            }

            impl AddAssign<$VecN> for $VecN {
                fn add_assign(&mut self, other: $VecN) {
                    *self = $VecN {
                        $(
                            $field: self.$field + other.$field,
                        )+
                    }
                }
            }

            impl Sub<$VecN> for $VecN {
                type Output = Self;

                fn sub(self, other: $VecN) -> Self {
                    $VecN {
                        $(
                            $field: self.$field - other.$field,
                        )+
                    }
                }
            }

            impl SubAssign<$VecN> for $VecN {
                fn sub_assign(&mut self, other: $VecN) {
                    *self = $VecN {
                        $(
                            $field: self.$field - other.$field,
                        )+
                    }
                }
            }

            impl Mul<$VecN> for $VecN {
                type Output = Self;

                fn mul(self, other: $VecN) -> Self {
                    $VecN {
                        $(
                            $field: self.$field * other.$field,
                        )+
                    }
                }
            }

            impl MulAssign<$VecN> for $VecN {
                fn mul_assign(&mut self, other: $VecN) {
                    *self = $VecN {
                        $(
                            $field: self.$field * other.$field,
                        )+
                    }
                }
            }

            impl Mul<f32> for $VecN {
                type Output = Self;

                fn mul(self, other: f32) -> Self {
                    $VecN {
                        $(
                            $field: self.$field * other,
                        )+
                    }
                }
            }

           impl MulAssign<f32> for $VecN {
                fn mul_assign(&mut self, other: f32) {
                    *self = $VecN {
                        $(
                            $field: self.$field * other,
                        )+
                    }
                }
            }

            impl Div<f32> for $VecN {
                type Output = Self;

                fn div(self, other: f32) -> Self {
                    $VecN {
                        $(
                            $field: self.$field / other,
                        )+
                    }
                }
            }

            impl DivAssign<f32> for $VecN {
                fn div_assign(&mut self, other: f32) {
                    *self = $VecN {
                        $(
                            $field: self.$field / other,
                        )+
                    }
                }
            }

            impl Mul<$VecN> for f32 {
                type Output = $VecN;

                fn mul(self, other: $VecN) -> $VecN {
                    $VecN {
                        $(
                            $field: other.$field * self,
                        )+
                    }
                }
            }

            impl Neg for $VecN {
                type Output = Self;

                fn neg(self) -> Self {
                    $VecN {
                        $(
                            $field: -self.$field,
                        )+
                    }
                }
            }

            impl Index<usize> for $VecN {
                type Output = f32;

                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        $($dimensions => &self.$field,)+
                        _ => panic!("PANIC. Out of bonds access on Vector: {:?}\nWith index: {}", self, index),
                    }
                }
            }

            impl IndexMut<usize> for $VecN {
                fn index_mut(&mut self, index: usize) -> &mut f32 {
                    match index {
                        $($dimensions => &mut self.$field,)+
                        _ => panic!("PANIC. Out of bonds access on Vector: {:?}\nWith index: {}", self, index),
                    }
                }
            }
    };
}
