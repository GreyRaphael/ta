use crate::utils::is_nan_or_inf;
use pyo3::prelude::*;
use std::f64::NAN;

#[pyclass]
pub struct Deltaer {
    first_value: Option<f64>,
}

#[pymethods]
impl Deltaer {
    #[new]
    pub fn new() -> Self {
        Self { first_value: None }
    }

    pub fn update(&mut self, x: f64) -> f64 {
        if is_nan_or_inf(x) {
            NAN
        } else {
            if self.first_value.is_none() {
                self.first_value = Some(x);
                0.0 // Delta is zero at the first value
            } else {
                x - self.first_value.unwrap()
            }
        }
    }
}
