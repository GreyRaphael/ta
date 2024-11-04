use pyo3::prelude::*;
use std::f64::NAN;

#[pyclass]
pub struct Container {
    buf: Vec<f64>,
    head_idx: usize,
    tail_idx: usize,
}

#[pymethods]
impl Container {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![NAN; n],
            head_idx: 0,
            tail_idx: 0,
        }
    }

    pub fn update(&mut self, new_val: f64) {
        self.tail_idx = self.head_idx;
        self.buf[self.tail_idx] = new_val;
        self.head_idx = (self.head_idx + 1) % self.buf.len();
    }

    pub fn get(&self, idx: usize) -> f64 {
        // idx=0 is head; idx=n-1 is tail
        self.buf[(self.head_idx + idx) % self.buf.len()]
    }

    pub fn head(&self) -> f64 {
        self.buf[self.head_idx]
    }

    pub fn tail(&self) -> f64 {
        self.buf[self.tail_idx]
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }
}
