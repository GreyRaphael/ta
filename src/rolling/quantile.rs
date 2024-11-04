use super::container::Container;
use crate::utils::is_nan_or_inf;
use pyo3::prelude::*;
use std::f64::NAN;

#[pyclass]
pub struct Quantiler {
    container: Container,
    dataset: Vec<f64>,
    nan_count: usize,
    quantile: f64,
}

#[pymethods]
impl Quantiler {
    #[new]
    pub fn new(n: usize, quantile: f64) -> Self {
        Self {
            container: Container::new(n),
            dataset: Vec::new(),
            nan_count: n,
            quantile,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let old_val = self.container.head();
        self.container.update(new_val);

        // Update nan_count and dataset based on new_val
        if is_nan_or_inf(new_val) {
            self.nan_count += 1;
        } else {
            let pos = self
                .dataset
                .binary_search_by(|v| v.partial_cmp(&new_val).unwrap())
                .unwrap_or_else(|e| e);
            self.dataset.insert(pos, new_val);
        }

        // Update nan_count and dataset based on old_val
        if is_nan_or_inf(old_val) {
            self.nan_count -= 1;
        } else {
            let pos = self
                .dataset
                .binary_search_by(|v| v.partial_cmp(&old_val).unwrap())
                .unwrap();
            self.dataset.remove(pos);
        }

        if self.nan_count > 0 {
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
