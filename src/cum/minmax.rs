use pyo3::prelude::*;
use std::f64::{INFINITY, NEG_INFINITY};

#[pyclass]
pub struct Maxer {
    max: f64,
}

#[pymethods]
impl Maxer {
    #[new]
    pub fn new() -> Self {
        Self { max: NEG_INFINITY }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        if new_val > self.max {
            self.max = new_val;
        }
        self.max
    }
}

#[pyclass]
pub struct Miner {
    min: f64,
}

#[pymethods]
impl Miner {
    #[new]
    pub fn new() -> Self {
        Self { min: INFINITY }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        if new_val < self.min {
            self.min = new_val;
        }
        self.min
    }
}
