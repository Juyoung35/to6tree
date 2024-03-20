macro_rules! ease {
    ($mod_name: ident, $ease_in:expr) => {
    // ($mod_name: ident, $ease_in:expr, $ease_out:expr, $ease_in_out:expr) => {
        pub mod $mod_name {
        use num_traits::{Float, FloatConst, FromPrimitive};
            #[inline]
            pub fn ease_in<T: Float + FloatConst + FromPrimitive>(x: T) -> T {
                $ease_in(x)
            }
            // #[inline]
            // pub fn ease_out<T: Float>(x: T) -> T {
            //     $ease_out(x)
            // }
            // #[inline]
            // pub fn ease_in_out<T: Float>(x: T) -> T {
            //     $ease_in_out(x)
            // }
        }
    }
}

use num_traits::{Float, FloatConst, FromPrimitive};
pub fn f<T: Float + FloatConst + FromPrimitive>(x: f64) -> T {
    T::from(x).unwrap()
}
ease!(sine,
    |x| crate::f::<T>(1.0) - T::cos((x * T::PI()) / crate::f::<T>(2.0))
);
// ease!(cubic)
// ease!(quint)
// ease!(circ)
// ease!(elastic)
// ease!(quad)
// ease!(quart)
// ease!(expo)
// ease!(back)
// ease!(bounce)