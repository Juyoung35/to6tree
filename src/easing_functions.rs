macro_rules! ease {
    ($mod_name: ident, $ease_in:expr, $ease_out:expr, $ease_in_out:expr) => {
        pub mod $mod_name {
            use num_traits::{Float, FloatConst, FromPrimitive};
            #[inline]
            pub fn ease_in<T: Float + FloatConst + FromPrimitive>(x: T) -> T {
                $ease_in(x)
            }
            #[inline]
            pub fn ease_out<T: Float + FloatConst + FromPrimitive>(x: T) -> T {
                $ease_out(x)
            }
            #[inline]
            pub fn ease_in_out<T: Float + FloatConst + FromPrimitive>(x: T) -> T {
                $ease_in_out(x)
            }
        }
    }
}

ease!(sine,
    |x: T| 1.0 - cos(PI * x / 2.0),
    |x: T| sin(PI * x / 2.0),
    |x: T| -(cos(PI * x) - 1.0) / 2.0,
);
ease!()