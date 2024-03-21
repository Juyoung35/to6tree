macro_rules! ease {
    ($mod_name: ident, $ease_in:expr, $ease_out:expr, $ease_in_out:expr) => {
        pub mod $mod_name {
            use num_traits::{Float, FloatConst, FromPrimitive};
            #[inline]
            pub fn ease_in<T: Float + FloatConst + FromPrimitive>(x) -> T {
                $ease_in(x)
            }
            #[inline]
            pub fn ease_out<T: Float + FloatConst + FromPrimitive>(x) -> T {
                $ease_out(x)
            }
            #[inline]
            pub fn ease_in_out<T: Float + FloatConst + FromPrimitive>(x) -> T {
                $ease_in_out(x)
            }
        }
    }
}

ease!(sine,
    |x| 1.0 - cos(PI * x / 2.0),
    |x| sin(PI * x / 2.0),
    |x| -(cos(PI * x) - 1.0) / 2.0,
);
ease!(cubic,
    |x| x.powi(3),
    |x| 1.0 - (1.0 - x).powi(3),
    |x| if x < 0.5 {
        4.0 * x.powi(3)
    } else {
        1.0 - (-2.0 * x + 2.0).powi(3) / 2.0
    },
);
ease!(quint,
    |x| x.powi(5),
    |x| 1.0 - (1.0 - x).powi(5),
    |x| if x < 0.5 {
        16.0 * x.powi(5)
    } else {
        1.0 - (-2.0 * x + 2.0).powi(5) / 2.0
    },
);
ease!(circ
    |x| 1.0 - sqrt(1.0 - x.powi(2)),
    |x| sqrt(1.0 - (x - 1.0).powi(2)),
    |x| if x < 0.5 {
        1.0 - sqrt(1.0 - (2.0 * x).powi(2))
    } else {
        sqrt(1.0 - (-2.0 * x + 2.0).powi(2)) + 1.0
    } / 2.0
);
ease!(elastic,
    |x| if_zero_one_else(x, |y| -2.0.powf(10.0 * y - 10.0) * sin((10.0 * y - 10.75) * 2.0 * PI / 3.0)),
    |x| if_zero_one_else(x, |y| 2.0.powf(-10.0 * y) * sin ((10.0 * y - 10.75) * 2.0 * PI / 3.0) + 1.0),
    |x| if_zero_one_else(x, if x < 0.5 {
        |y| -2.0.powf(20.0 * y - 10.0) * sin((20.0 * y - 11.125) * 2.0 * PI / 4.5) / 2.0
    } else {
        |y| 2.0.powf(-20.0 * y) * sin ((20.0 * y - 11.125) * 2.0 * PI / 4.5) / 2.0 + 1.0
    }),
);
ease!(cubic,
    |x| x.powi(2),
    |x| 1.0 - (1.0 - x).powi(2),
    |x| if x < 0.5 {
        2.0 * x.powi(2)
    } else {
        1.0 - (-2.0 * x + 2.0).powi(2) / 2.0
    },
);
ease!(quart,
    |x| x.powi(4),
    |x| 1.0 - (1.0 - x).powi(4),
    |x| if x < 0.5 {
        8.0 * x.powi(4)
    } else {
        1.0 - (-2.0 * x + 2.0).powi(4) / 2.0
    },
);
ease!(expo,
    |x| if_zero_one_else(x, |y| 2.0.powf(10.0 * y - 10.0)),
    |x| if_zero_one_else(x, |y| 1.0 - 2.0.powf(-10.0 * y)),
    |x| if_zero_one_else(x, if x < 0.5 {
        |y| 2.0.powf(20.0 * y - 10.0)
    } else {
        |y| 2.0 - 2.0.powf(-20.0 * y + 10.0)
    } / 2.0),
);
ease!(back,
    |x| 2.70158 * x.powi(3) - 1.70158 * x.powi(2),
    |x| 1.0 + 2.70158 * (x - 1.0).powi(3) + 1.70158 * (x - 1.0).powi(2),
    |x| if x < 0.5 {
        (2.0 * x).powi(2) * (1.70158 * 1.525 + 1.0) * 2.0 * x - 1.70158 * 1.525
    } else {
        (2.0 * x - 2.0).powi(2) * ((1.70158 * 1.525 + 1.0) * (x * 2.0 - 2.0) + 1.70158 * 1.525) + 2.0
    } / 2.0,
);
ease!(bounce,
    |x| 1.0 - ease_out_bounce(1.0 - x),
    |x| ease_out_bounce(x),
    |x| if x < 0.5 {
        (1.0 - ease_out_bounce(1.0 - 2.0 * x))
    } else {
        (1.0 + ease_out_bounce(2.0 * x - 1.0))
    } / 2.0,
)

fn if_zero_one_else<T, C>(x, closure: C) -> T
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
fn ease_out_bounce<T: Float>(x) -> T {
    if x < 1.0 / 2.75 {
        7.5625 * x.powi(2)
    } else if x < 2.0 / 2.75 {
        7.5625 * (x - 1.5 / 2.75) * (x - 1.5 / 2.75) + 0.75
    } else if x < 2.5 / 2.75 {
        7.5625 * (x - 2.25 / 2.75) * (x - 2.25 / 2.75) + 0.9375
    } else {
        7.5625 * (x - 2.625 / 2.75) * (x - 2.625 / 2.75) + 0.984375
    }
}