use num_traits::Float;

macro_rules ease {
    ($mod_name: ident, $ease_in:expr, $ease_out:expr, $ease_in_out:expr) => {
        pub mod $mod_name {
            #[inline]
            pub fn ease_in<T: Float>(x: T) -> T {
                $ease_in(x)
            }
            #[inline]
            pub fn ease_out<T: Float>(x: T) -> T {
                $ease_out(x)
            }
            #[inline]
            pub fn ease_in_out<T: Float>(x: T) -> T {
                $ease_in_out(x)
            }
        }
    }
}

ease!(sine)
ease!(cubic)
ease!(quint)
ease!(circ)
ease!(elastic)
ease!(quad)
ease!(quart)
ease!(expo)
ease!(back)
ease!(bounce)