mod shaping_functions;

#[macro_export]
macro_rules! float {
    ($x:expr) => {
        T::from_f32($x).unwrap()
    };
}
