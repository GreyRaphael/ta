use super::sum::{Meaner, Sumer};
use pyo3::prelude::*;

#[pyclass]
pub struct Stder {
    pub sumer: Sumer,
    pub sq_sumer: Sumer,
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

        let variance = (sq_sum - sum * sum / (self.n as f64)) / ((self.n - 1) as f64);
        variance.sqrt()
    }
}

#[pyclass]
pub struct Skewer {
    pub meaner: Meaner,
    pub cub_sumer: Sumer,
    pub sq_sumer: Sumer,
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
    pub sumer: Sumer,
    pub sq_sumer: Sumer,
    pub cub_sumer: Sumer,
    pub quad_sumer: Sumer,
    n: usize,
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
        let factor01 = ((self.n + 1) as f64 * self.n as f64)
            / ((self.n - 1) as f64 * (self.n - 2) as f64 * (self.n - 3) as f64);
        let factor02 = 3.0 * ((self.n - 1) as f64 * (self.n - 1) as f64)
            / ((self.n - 2) as f64 * (self.n - 3) as f64);
        (factor01 * value / variance.powi(2)) - factor02
    }
}
