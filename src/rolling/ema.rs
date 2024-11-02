use crate::utils::is_nan_or_inf;
use pyo3::prelude::*;
use std::f64::NAN;

// Cumulative Exponential Moving Average (EMA) over all data points.
// Since EMA is already cumulative in nature, we can use it as is.

#[pyclass]
pub struct EMAer {
    alpha: f64,
    ema: Option<f64>,
}

#[pymethods]
impl EMAer {
    #[new]
    pub fn new(n: usize) -> Self {
        let alpha = 2.0 / (n as f64 + 1.0);
        Self { alpha, ema: None }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        if !is_nan_or_inf(new_val) {
            if let Some(prev_ema) = self.ema {
                self.ema = Some(prev_ema * (1.0 - self.alpha) + new_val * self.alpha);
            } else {
                self.ema = Some(new_val);
            }
        }
        self.ema.unwrap_or(NAN)
    }
}
