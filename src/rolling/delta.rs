use pyo3::prelude::*;
use std::f64::NAN;

#[pyclass]
pub struct Deltaer {
    buf: Vec<f64>,
    pub n: usize,
    pub cur_idx: usize,
    pub head_idx: usize,
}

#[pymethods]
impl Deltaer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![NAN; n],
            n,
            cur_idx: 0,
            head_idx: 0,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.head_idx = self.cur_idx;
        self.buf[self.head_idx] = new_val;
        self.cur_idx = (self.cur_idx + 1) % self.n;

        new_val - self.buf[self.cur_idx]
    }
}

#[pyclass]
pub struct Pctchanger {
    pub deltaer: Deltaer,
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
}
