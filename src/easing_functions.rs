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
    |x| T::from(1).unwrap() - T::cos(x * T::PI() / T::from(2).unwrap()),
    |x| T::sin(x * T::PI() / T::from(2).unwrap()),
    |x| -(T::cos(x * T::PI()) - T::from(1).unwrap()) / T::from(2).unwrap(),
);
ease!(cubic,
    |x| x.powi(3u32),
    |x| T::from(1).unwrap() - (T::from(1).unwrap() - x).powi(3u32),
    |x| if x < T::from(0.5).unwrap() {
        T::from(4).unwrap() * x.powi(3u32)
    } else {
        T::from(1).unwrap() - (-T::from(2).unwrap() * x + 2).powi(3u32) / T::from(2).unwrap()
    },
);
ease!(quint,
    |x| x.powi(5u32),
    |x| T::from(1).unwrap() - (T::from(1).unwrap() - x).powi(5u32),
    |x| if x < T::from(0.5).unwrap() {
        T::from(16).unwrap() * x.powi(3u32)
    } else {
        T::from(1).unwrap() - (-T::from(2).unwrap() * x + 2).powi(5u32) / T::from(2).unwrap()
    },
);
ease!(circ,
    |x| T::from(1).unwrap() - T::sqrt(T::from(1).unwrap() - x.powi(2u32)),
    |x| T::sqrt(T::from(1).unwrap() - (x - T::from(1).unwrap()).powi(2u32)),
    |x| if x < T::from(0.5).unwrap() {
        (T::from(1).unwrap() - T::sqrt(T::from(1).unwrap() - (T::from(2).unwrap() * x).powi(2u32))) / T::from(2).unwrap()
    } else {
        (T::sqrt(T::from(1).unwrap() - (-T::from(2).unwrap() * x + 2).powi(2u32) + T:: from(1).unwrap())) / T::from(2).unwrap()
    },
);
ease!(elastic,
    |x| if x == T::from(0).unwrap() {
        T::from(0).unwrap()
    } else if x == T::from(1).unwrap() {
        T::from(1).unwrap()
    } else {
        -T::from(2).unwrap().powf(T::from(10).unwrap() * x - T::from(10).unwrap()) * T::sin((x * T::from(10).unwrap() - T::from(10.75).unwrap()) * T::from(2).unwrap() * T::PI() / T::from(3).unwrap())
    },
    |x| if x == T::from(0).unwrap() {
        T::from(0).unwrap()
    } else if x == T::from(1).unwrap() {
        T::from(1).unwrap()
    } else {
        -T::from(2).unwrap().powf(T::from(10).unwrap() * x - T::from(10).unwrap()) * T::sin((x * T::from(10).unwrap() - T::from(10.75).unwrap()) * T::from(2).unwrap() * T::PI() / T::from(3).unwrap())
    },
);
// ease!(quad)
// ease!(quart)
// ease!(expo)
// ease!(back)
// ease!(bounce)