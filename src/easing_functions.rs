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
fn if_zero_one_else<T, C>(x: T, closure: C) -> T
where
    T: PartialEq<Float>,
    C: Fn(T) -> T,
{
    if x == 0.0 | x == 1.0 {
        x
    } else {
        closure(x)
    }
}

ease!(sine,
    |x: T| 1.0 - cos(PI * x / 2.0),
    |x: T| sin(PI * x / 2.0),
    |x: T| -(cos(PI * x) - 1.0) / 2.0,
);
ease!(cubic,
    |x: T| x.powi(3),
    |x: T| 1.0 - (1.0 - x).powi(3),
    |x: T| if x < 0.5 {
        4.0 * x.powi(3)
    } else {
        1.0 - (-2.0 * x + 2.0).powi(3) / 2.0
    },
);
ease!(quint,
    |x: T| x.powi(5),
    |x: T| 1.0 - (1.0 - x).powi(5),
    |x: T| if x < 0.5 {
        16.0 * x.powi(5)
    } else {
        1.0 - (-2.0 * x + 2.0).powi(5) / 2.0
    },
);
ease!(circ
    |x: T| 1.0 - sqrt(1.0 - x.powi(2)),
    |x: T| sqrt(1.0 - (x - 1.0).powi(2)),
    |x: T| if x < 0.5 {
        1.0 - sqrt(1.0 - (2.0 * x).powi(2))
    } else {
        sqrt(1.0 - (-2.0 * x + 2.0).powi(2)) + 1.0
    } / 2.0
);
ease!(elastic,
    |x: T| if_zero_one_else(x, |y| -2.0.powf(10.0 * y - 10.0) * sin((10.0 * y - 10.75) * 2.0 * PI / 3.0)),
    |x: T| if_zero_one_else(x, |y| 2.0.powf(-10.0 * y) * sin ((10.0 * y - 10.75) * 2.0 * PI / 3.0) + 1.0),
    |x: T| if_zero_one_else(x, if x < 0.5 {
        |y| -2.0.powf(20.0 * y - 10.0) * sin((20.0 * y - 11.125) * 2.0 * PI / 4.5) / 2.0
    } else {
        |y| 2.0.powf(-20.0 * y) * sin ((20.0 * y - 11.125) * 2.0 * PI / 4.5) / 2.0 + 1.0
    }),
);
ease!(cubic,
    |x: T| x.powi(2),
    |x: T| 1.0 - (1.0 - x).powi(2),
    |x: T| if x < 0.5 {
        2.0 * x.powi(2)
    } else {
        1.0 - (-2.0 * x + 2.0).powi(2) / 2.0
    },
);
ease!(quart,
    |x: T| x.powi(2),
    |x: T| 1.0 - (1.0 - x).powi(2),
    |x: T| if x < 0.5 {
        2.0 * x.powi(2)
    } else {
        1.0 - (-2.0 * x + 2.0).powi(2) / 2.0
    },
);