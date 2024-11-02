use pyo3::prelude::*;
use std::f64::NAN;

#[pyclass]
pub struct Deltaer {
    buf: Vec<f64>,
    cur_idx: usize,
    head_idx: usize,
}

#[pymethods]
impl Deltaer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![NAN; n],
            cur_idx: 0,
            head_idx: 0,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.head_idx = self.cur_idx;
        self.buf[self.head_idx] = new_val;
        self.cur_idx = (self.cur_idx + 1) % self.buf.len();

        new_val - self.buf[self.cur_idx]
    }

    pub fn get(&self, idx: usize) -> f64 {
        self.buf[(self.head_idx + idx) % self.buf.len()]
    }
}

#[pyclass]
pub struct Pctchanger {
    deltaer: Deltaer,
}

#[pymethods]
impl Pctchanger {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            deltaer: Deltaer::new(n),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let diff = self.deltaer.update(new_val);
        diff / (new_val - diff)
    }

    pub fn get(&self, idx: usize) -> f64 {
        self.deltaer.get(idx)
    }
}
