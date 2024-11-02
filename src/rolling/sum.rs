use pyo3::prelude::*;
use std::f64::NAN;

#[pyclass]
pub struct Sumer {
    buf: Vec<f64>,
    pub n: usize,
    cur_idx: usize,
    nan_count: usize,
    sum: f64,
}

#[pymethods]
impl Sumer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![NAN; n],
            n,
            cur_idx: 0,
            nan_count: n,
            sum: 0.0,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let old_val = self.buf[self.cur_idx];
        self.buf[self.cur_idx] = new_val;
        self.cur_idx = (self.cur_idx + 1) % self.n;

        if is_nan_or_inf(new_val) {
            self.nan_count += 1;
        } else {
            self.sum += new_val;
        }

        if is_nan_or_inf(old_val) {
            self.nan_count -= 1;
        } else {
            self.sum -= old_val;
        }

        if self.nan_count > 0 {
            NAN
        } else {
            self.sum
        }
    }
}

fn is_nan_or_inf(x: f64) -> bool {
    x.is_nan() || x.is_infinite()
}
