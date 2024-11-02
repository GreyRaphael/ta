use crate::utils::is_nan_or_inf;
use pyo3::prelude::*;
use std::f64::NAN;

#[pyclass]
pub struct Quantiler {
    dataset: Vec<f64>,
    quantile: f64,
}

#[pymethods]
impl Quantiler {
    #[new]
    pub fn new(quantile: f64) -> Self {
        Self {
            dataset: Vec::new(),
            quantile,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        if !is_nan_or_inf(new_val) {
            let pos = self
                .dataset
                .binary_search_by(|v| v.partial_cmp(&new_val).unwrap())
                .unwrap_or_else(|e| e);
            self.dataset.insert(pos, new_val);
            self.get_quantile()
        } else {
            NAN
        }
    }

    pub fn get_quantile(&self) -> f64 {
        if self.dataset.is_empty() {
            NAN
        } else {
            let index = (self.dataset.len() - 1) as f64 * self.quantile;
            let lower_index = index.floor() as usize;
            let fraction = index - lower_index as f64;

            let lower_value = self.dataset[lower_index];
            let upper_value = if lower_index + 1 < self.dataset.len() {
                self.dataset[lower_index + 1]
            } else {
                lower_value
            };

            lower_value + fraction * (upper_value - lower_value)
        }
    }
}
