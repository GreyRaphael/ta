use pyo3::prelude::*;
use super::sum::Sumer;

#[pyclass]
pub struct Meaner {
    sumer: Sumer,
}

#[pymethods]
impl Meaner {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            sumer: Sumer::new(n),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let sum = self.sumer.update(new_val);
        sum / self.sumer.n as f64
    }
}
