
pub fn is_nan_or_inf(x: f64) -> bool {
    x.is_nan() || x.is_infinite()
}