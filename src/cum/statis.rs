use crate::utils::is_nan_or_inf;
use pyo3::prelude::*;
use std::f64::NAN;

#[pyclass]
pub struct Counter {
    count: f64,
}

#[pymethods]
impl Counter {
    #[new]
    pub fn new() -> Self {
        Self { count: 0.0 }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        if is_nan_or_inf(new_val) {
            NAN
        } else {
            self.count += 1.0;
            self.count
        }
    }
}

#[pyclass]
pub struct Sumer {
    sum: f64,
}

#[pymethods]
impl Sumer {
    #[new]
    pub fn new() -> Self {
        Self { sum: 0.0 }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        if is_nan_or_inf(new_val) {
            NAN
        } else {
            self.sum += new_val;
            self.sum
        }
    }
}

#[pyclass]
pub struct Meaner {
    count: f64,
    sum: f64,
}

#[pymethods]
impl Meaner {
    #[new]
    pub fn new() -> Self {
        Self {
            count: 0.0,
            sum: 0.0,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        if is_nan_or_inf(new_val) {
            NAN
        } else {
            self.count += 1.0;
            self.sum += new_val;
            self.sum / self.count
        }
    }
}

#[pyclass]
pub struct Stder {
    count: f64,
    sumer: Sumer,
    sq_sumer: Sumer,
}

#[pymethods]
impl Stder {
    #[new]
    pub fn new() -> Self {
        Self {
            count: 0.0,
            sumer: Sumer::new(),
            sq_sumer: Sumer::new(),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        if is_nan_or_inf(new_val) {
            NAN
        } else {
            self.count += 1.0;
            let sum = self.sumer.update(new_val);
            let sq_sum = self.sq_sumer.update(new_val * new_val);

            let variance = (sq_sum - sum * sum / self.count as f64) / (self.count as f64 - 1.0);
            variance.sqrt()
        }
    }
}
