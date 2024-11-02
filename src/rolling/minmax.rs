use pyo3::prelude::*;
use std::f64::NAN;

#[pyclass]
pub struct Maxer {
    buf: Vec<f64>,
    cur_idx: usize,
    nan_count: usize,
}

#[pymethods]
impl Maxer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![NAN; n],
            cur_idx: 0,
            nan_count: n,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let old_val = self.buf[self.cur_idx];
        self.buf[self.cur_idx] = new_val;
        self.cur_idx = (self.cur_idx + 1) % self.buf.len();

        if new_val.is_nan() {
            self.nan_count += 1;
        }

        if old_val.is_nan() {
            self.nan_count -= 1;
        }

        if self.nan_count > 0 {
            NAN
        } else {
            self.buf
                .iter()
                .fold(NAN, |max, x| if *x < max { max } else { *x })
        }
    }
}

#[pyclass]
pub struct Miner {
    buf: Vec<f64>,
    cur_idx: usize,
    nan_count: usize,
}

#[pymethods]
impl Miner {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![NAN; n],
            cur_idx: 0,
            nan_count: n,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let old_val = self.buf[self.cur_idx];
        self.buf[self.cur_idx] = new_val;
        self.cur_idx = (self.cur_idx + 1) % self.buf.len();

        if new_val.is_nan() {
            self.nan_count += 1;
        }

        if old_val.is_nan() {
            self.nan_count -= 1;
        }

        if self.nan_count > 0 {
            NAN
        } else {
            self.buf
                .iter()
                .fold(NAN, |max, x| if *x > max { max } else { *x })
        }
    }
}
