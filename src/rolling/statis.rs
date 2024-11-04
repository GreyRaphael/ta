use super::container::Container;
use crate::utils::is_nan_or_inf;
use pyo3::prelude::*;
use std::f64::NAN;

#[pyclass]
pub struct Sumer {
    container: Container,
    nan_count: usize,
    sum: f64,
}

#[pymethods]
impl Sumer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            container: Container::new(n),
            nan_count: n,
            sum: 0.0,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let old_val = self.container.head();
        self.container.update(new_val);

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
        self.sumer.update(new_val) / self.sumer.container.len() as f64
    }
}

#[pyclass]
pub struct Stder {
    sumer: Sumer,
    sq_sumer: Sumer,
    n: usize,
}

#[pymethods]
impl Stder {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            sumer: Sumer::new(n),
            sq_sumer: Sumer::new(n),
            n,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let sum = self.sumer.update(new_val);
        let sq_sum = self.sq_sumer.update(new_val * new_val);

        let variance = (sq_sum - sum * sum / self.n as f64) / (self.n as f64 - 1.0);
        variance.sqrt()
    }
}

#[pyclass]
pub struct Skewer {
    meaner: Meaner,
    sq_sumer: Sumer,
    cub_sumer: Sumer,
    n: usize,
}

#[pymethods]
impl Skewer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            meaner: Meaner::new(n),
            sq_sumer: Sumer::new(n),
            cub_sumer: Sumer::new(n),
            n,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let mean = self.meaner.update(new_val);
        let sq_sum = self.sq_sumer.update(new_val * new_val);
        let cub_sum = self.cub_sumer.update(new_val * new_val * new_val);

        let variance = sq_sum / self.n as f64 - mean * mean;

        (cub_sum / self.n as f64 - 3.0 * mean * variance - mean.powi(3)) / variance.powf(1.5)
    }
}

#[pyclass]
pub struct Kurter {
    sumer: Sumer,
    sq_sumer: Sumer,
    cub_sumer: Sumer,
    quad_sumer: Sumer,
    n: usize,
    factor01: f64,
    factor02: f64,
}

#[pymethods]
impl Kurter {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            sumer: Sumer::new(n),
            sq_sumer: Sumer::new(n),
            cub_sumer: Sumer::new(n),
            quad_sumer: Sumer::new(n),
            n,
            factor01: (n * (n + 1)) as f64 / ((n - 1) * (n - 2) * (n - 3)) as f64,
            factor02: ((n - 1) * (n - 1)) as f64 / ((n - 2) * (n - 3)) as f64,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let quad_sum = self.quad_sumer.update(new_val.powi(4));
        let cub_sum = self.cub_sumer.update(new_val.powi(3));
        let sq_sum = self.sq_sumer.update(new_val.powi(2));
        let sum = self.sumer.update(new_val);
        let mean = sum / self.n as f64;
        let variance =
            (sq_sum - 2.0 * sum * mean + self.n as f64 * mean.powi(2)) / (self.n as f64 - 1.0);

        let value = quad_sum - 4.0 * cub_sum * mean + 6.0 * sq_sum * mean.powi(2)
            - 4.0 * sum * mean.powi(3)
            + self.n as f64 * mean.powi(4);

        self.factor01 * value / variance.powi(2) - self.factor02
    }
}
