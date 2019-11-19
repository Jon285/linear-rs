//implementation of all basic ops to the matrix structs
macro_rules! impl_mat_ops {
    //basic ops
    ($MatN:ident, $field:ident, $dimension:expr, $indextype:ty) => {
        // use crate::FloatScalar;
        use std::ops::*;

        impl<T: FloatScalar> Add<$MatN<T>> for $MatN<T> {
            type Output = $MatN<T>;

            fn add(self, other: $MatN<T>) -> Self::Output {
                let mut ret = $MatN::default();

                for i in 0..$dimension {
                    for j in 0..$dimension {
                        ret.$field[i][j] = self.$field[i][j] + other.$field[i][j];
                    }
                }
                ret
            }
        }

        impl<T: FloatScalar> AddAssign<$MatN<T>> for $MatN<T> {
            fn add_assign(&mut self, other: $MatN<T>) {
                for i in 0..$dimension {
                    for j in 0..$dimension {
                        self.$field[i][j] += other.$field[i][j];
                    }
                }
            }
        }

        impl<T: FloatScalar> Sub<$MatN<T>> for $MatN<T> {
            type Output = Self;

            fn sub(self, other: $MatN<T>) -> Self {
                let mut ret = $MatN::default();

                for i in 0..$dimension {
                    for j in 0..$dimension {
                        ret.$field[i][j] = self.$field[i][j] - other.$field[i][j];
                    }
                }
                ret
            }
        }

        impl<T: FloatScalar> SubAssign<$MatN<T>> for $MatN<T> {
            fn sub_assign(&mut self, other: $MatN<T>) {
                for i in 0..$dimension {
                    for j in 0..$dimension {
                        self.$field[i][j] -= other.$field[i][j];
                    }
                }
            }
        }

        impl<T: FloatScalar> Mul<T> for $MatN<T> {
            type Output = Self;

            fn mul(self, other: T) -> Self {
                let mut ret = $MatN::default();

                for i in 0..$dimension {
                    for j in 0..$dimension {
                        ret.$field[i][j] = self.$field[i][j] * other;
                    }
                }
                ret
            }
        }

        impl<T: FloatScalar> Mul<$MatN<T>> for $MatN<T> {
            type Output = Self;

            fn mul(self, rhs: $MatN<T>) -> Self {
                let mut ret = $MatN::zero();

                for i in 0..$dimension {
                    for j in 0..$dimension {
                        for k in 0..$dimension {
                            ret[i][j] += self[k][j] * rhs[i][k];
                        }
                    }
                }
                ret
            }
        }

        impl<T> Index<usize> for $MatN<T> {
            type Output = $indextype;

            fn index(&self, index: usize) -> &Self::Output {
                &self.$field[index]
            }
        }

        impl<T> IndexMut<usize> for $MatN<T> {
            fn index_mut(&mut self, index: usize) -> &mut $indextype {
                &mut self.$field[index]
            }
        }
    };

    //Matrix x Vector multiplication
    ($MatN:ident, $VecN:ident, $dimension:expr) => {
        use std::ops::Mul;

        impl<T: FloatScalar> Mul<$VecN<T>> for $MatN<T> {
            type Output = $VecN<T>;

            fn mul(self, other: $VecN<T>) -> $VecN<T> {
                let mut ret = $VecN::default();

                for i in 0..$dimension {
                    for j in 0..$dimension {
                        ret[i] += other[j] * self[j][i];
                    }
                }
                ret
            }
        }
    };
}
