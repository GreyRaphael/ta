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
                .fold(NAN, |cur_max, x| if *x <= cur_max { cur_max } else { *x })
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
                .fold(NAN, |cur_min, x| if *x >= cur_min { cur_min } else { *x })
        }
    }
}

#[pyclass]
pub struct MinMaxer {
    buf: Vec<f64>,
    cur_idx: usize,
    nan_count: usize,
}

#[pymethods]
impl MinMaxer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![NAN; n],
            cur_idx: 0,
            nan_count: n,
        }
    }

    pub fn update(&mut self, new_val: f64) -> (f64, f64) {
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
            // (min_val, max_val)
            (NAN, NAN)
        } else {
            self.buf.iter().fold((NAN, NAN), |(cur_min, cur_max), x| {
                let temp_min;
                let temp_max;
                if *x >= cur_min {
                    temp_min = cur_min;
                } else {
                    temp_min = *x;
                }

                if *x <= cur_max {
                    temp_max = cur_max;
                } else {
                    temp_max = *x;
                }
                (temp_min, temp_max)
            })
        }
    }
}

#[pyclass]
pub struct MaxIndexer {
    buf: Vec<f64>,
    cur_idx: usize,
    nan_count: usize,
}

#[pymethods]
impl MaxIndexer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![NAN; n],
            cur_idx: 0,
            nan_count: n,
        }
    }

    pub fn update(&mut self, new_val: f64) -> (usize, f64) {
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
            (usize::MAX, NAN)
        } else {
            self.buf
                .iter()
                .enumerate()
                .fold((usize::MAX, NAN), |(cur_idx, cur_max), (idx, x)| {
                    if *x <= cur_max {
                        (cur_idx, cur_max)
                    } else {
                        (idx, *x)
                    }
                })
        }
    }
}

#[pyclass]
pub struct MinIndexer {
    buf: Vec<f64>,
    cur_idx: usize,
    nan_count: usize,
}

#[pymethods]
impl MinIndexer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![NAN; n],
            cur_idx: 0,
            nan_count: n,
        }
    }

    pub fn update(&mut self, new_val: f64) -> (usize, f64) {
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
            (usize::MAX, NAN)
        } else {
            self.buf
                .iter()
                .enumerate()
                .fold((usize::MAX, NAN), |(cur_idx, cur_min), (idx, x)| {
                    if *x >= cur_min {
                        (cur_idx, cur_min)
                    } else {
                        (idx, *x)
                    }
                })
        }
    }
}

#[pyclass]
pub struct MinMaxIndexer {
    buf: Vec<f64>,
    cur_idx: usize,
    nan_count: usize,
}

#[pymethods]
impl MinMaxIndexer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            buf: vec![NAN; n],
            cur_idx: 0,
            nan_count: n,
        }
    }

    pub fn update(&mut self, new_val: f64) -> (usize, f64, usize, f64) {
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
            // (min_idx, min_val, max_indx, max_val)
            (usize::MAX, NAN, usize::MAX, NAN)
        } else {
            self.buf.iter().enumerate().fold(
                (usize::MAX, NAN, usize::MAX, NAN),
                |(cur_mini, cur_min, cur_maxi, cur_max), (idx, x)| {
                    let temp_min;
                    let temp_max;
                    if *x >= cur_min {
                        temp_min = (cur_mini, cur_min);
                    } else {
                        temp_min = (idx, *x);
                    }

                    if *x <= cur_max {
                        temp_max = (cur_maxi, cur_max);
                    } else {
                        temp_max = (idx, *x);
                    }

                    (temp_min.0, temp_min.1, temp_max.0, temp_max.1)
                },
            )
        }
    }
}
