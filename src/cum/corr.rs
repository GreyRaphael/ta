use super::statis::Meaner;
use crate::utils::is_nan_or_inf;
use pyo3::prelude::*;
use std::f64::NAN;

#[pyclass]
pub struct Correlationer {
    x_meaner: Meaner,
    x_sq_meaner: Meaner,
    y_meaner: Meaner,
    y_sq_meaner: Meaner,
    xy_meaner: Meaner,
}

#[pymethods]
impl Correlationer {
    #[new]
    pub fn new() -> Self {
        Self {
            x_meaner: Meaner::new(),
            x_sq_meaner: Meaner::new(),
            y_meaner: Meaner::new(),
            y_sq_meaner: Meaner::new(),
            xy_meaner: Meaner::new(),
        }
    }

    pub fn update(&mut self, x: f64, y: f64) -> f64 {
        if is_nan_or_inf(x) || is_nan_or_inf(y) {
            NAN
        } else {
            let mean_x = self.x_meaner.update(x);
            let mean_x_sq = self.x_sq_meaner.update(x * x);
            let mean_y = self.y_meaner.update(y);
            let mean_y_sq = self.y_sq_meaner.update(y * y);
            let mean_xy = self.xy_meaner.update(x * y);

            let sigmax_sq = (mean_x_sq - mean_x.powi(2)).sqrt();
            let sigmay_sq = (mean_y_sq - mean_y.powi(2)).sqrt();

            (mean_xy - mean_x * mean_y) / (sigmax_sq * sigmay_sq)
        }
    }
}
