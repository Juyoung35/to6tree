pub mod polynomial;
// pub mod exponential;
// pub mod circular_and_elliptical;
// pub mod bezier_and_parametric;

#[macro_export]
macro_rules! float {
    ($x:expr) => {
        T::from_f32($x).unwrap()
    };
}
