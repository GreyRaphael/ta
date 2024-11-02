pub fn is_nan_or_inf(x: f64) -> bool {
    // x - x != 0.0
    x.is_nan() || x.is_infinite()
}

pub fn sign<T: PartialOrd + From<i8>>(x: T) -> i8 {
    if x > T::from(0) {
        1
    } else if x < T::from(0) {
        -1
    } else {
        0
    }
}
