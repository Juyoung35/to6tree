use num_traits::{Float, FromPrimitive, Zero, One};
use crate::float;

pub fn blinn_wyvill_approximation_to_the_raised_inverted_cosine<T: Float + FromPrimitive>(x: T) -> T {
    // sin function approximation
    let x2 = x * x;
    let x4 = x2 * x2;
    let x6 = x4 * x2;

    let fa = float!(4.0 / 9.0);
    let fb = float!(17.0 / 9.0);
    let fc = float!(22.0 / 9.0);

    let y = fa * x6 - fb * x4 + fc * x2;
    y
}

fn double_cubic_seat<T: Float + FromPrimitive + Zero + One>(x: T, a: T, b: T) -> T {
    // The curves meet with a horizontal inflection point at the control coordinate (a,b) in the unit square.
    let min_a = T::zero() + T::epsilon();
    let max_a = T::one() - T::epsilon();
    let min_b = T::zero();
    let max_b = T::one();

    let a = T::min(max_a, T::max(min_a, a));
    let b = T::min(max_b, T::max(min_b, b));

    let y = if x <= a {
        b - b * T::powi(T::one() - x / a, 3)
    } else {
        b + (T::one() - b) * T::powi((x - a) / (T::one() - a), 3)
    };
    y
}

fn double_cubic_seat_with_linear_blend<T: Float + FromPrimitive + Zero + One>(x: T, a: T, b: T) -> T {
    // This modified version of the Double-Cubic Seat function uses a single variable to control the location of its inflection
    // point along the diagonal of the unit square. A second parameter is used to blend this curve with the Identity
    // Function (y=x). Here, we use the variable b to control the amount of this blend, which has the effect of tilting the
    // slope of the curve's plateau in the vicinity of its inflection point. The adjustable flattening around the inflection point
    // makes this a useful shaping function for lensing or magnifying evenly-spaced data.
    let min_a = T::zero() + T::epsilon();
    let max_a = T::one() - T::epsilon();
    let min_b = T::zero();
    let max_b = T::one();

    let a = T::min(max_a, T::max(min_a, a));
    let b = T::min(max_b, T::max(min_b, b));
    let b = T::one() - b; //reverse for intelligibility.

    let y = if x <= a {
        b * x + (T::one() - b) * a * (T::one() - T::powi(T::one() - x / a, 3))
    } else {
        b * x + (T::one() - b) * (a + (T::one() - a) * T::powi((x - a) / (T::one() - a), 3))
    };
    y
}

fn double_odd_polynomial_seat<T: Float + FromPrimitive + Zero + One>(x: T, a: T, b: T, n: i32) -> T {
    // The previous Double-Cubic Seat function can be generalized to a form which uses any odd integer exponent. In the
    // code below, the parameter n controls the flatness or breadth of the plateau region in the vicinity of the point (a,b). A
    // good working range for n is the set of whole numbers from 1 to about 20.
    let min_a = T::zero() + T::epsilon();
    let max_a = T::one() - T::epsilon();
    let min_b = T::zero();
    let max_b = T::one();

    let a = T::min(max_a, T::max(min_a, a));
    let b = T::min(max_b, T::max(min_b, b));

    let p = 2 * n + 1;
    let y = if x <= a {
        b - b * T::powi(T::one() - x / a, p)
    } else {
        b + (T::one() - b) * T::powi((x - a) / (T::one() - a), p)
    };
    y
}

fn symmetric_double_polynomial_sigmoid<T: Float + FromPrimitive + One>(x: T, n: i32) -> T {
    // The Symmetric Double-Polynomial Sigmoids presented here create an S-shape with flat tangents at 0 and 1, and
    // with the special property that f(0.5) = 0.5.
    let y = if n % 2 == 0{
        // even polynomial
        if x <= float!(0.5) {
            T::powi(float!(2.0) * x, n) / float!(2.0)
        } else {
            T::one() - T::powi(float!(2.0) * (x - T::one()), n) / float!(2.0)
        }
    } else {
        // odd polynomial
        if x <= float!(0.5) {
            T::powi(float!(2.0) * x, n) / float!(2.0)
        } else {
            T::one() + T::powi(float!(2.0) * (x - T::one()), n) / float!(2.0)
        }
    };
    y
}

fn quadratic_through_a_given_point<T: Float + FromPrimitive + Zero + One>(x: T, a: T, b: T) -> T {
    // This function defines an axis-aligned quadratic (parabola) which passes through a user-supplied point (a,b) in the
    // unit square. Caution: Not all points in the unit square will produce curves which pass through the locations (0,0) and (1,1).
    let min_a = T::zero() + T::epsilon();
    let max_a = T::one() - T::epsilon();
    let min_b = T::zero();
    let max_b = T::one();

    let a = T::min(max_a, T::max(min_a, a));
    let b = T::min(max_b, T::max(min_b, b));

    let large_a = (T::one() - b) / (T::one() - a) - (b / a);
    let large_b = (large_a * (a * a) - b) / a;
    let y = large_a * (x * x) - large_b * x;
    y
}
