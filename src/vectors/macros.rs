macro_rules! impl_vec_ops {
    ($VecN:ident, $($field:ident),+) => (

            impl Add<$VecN> for $VecN {
                type Output = $VecN;

                fn add(self, other: $VecN) -> Self::Output {
                    $VecN {
                        $(
                            $field: self.$field + other.$field,
                        )+
                    }
                }
            }

            impl Sub<$VecN> for $VecN {
                type Output = $VecN;

                fn sub(self, other: $VecN) -> Self::Output {
                    $VecN {
                        $(
                            $field: self.$field - other.$field,
                        )+
                    }
                }
            }

    )
}
