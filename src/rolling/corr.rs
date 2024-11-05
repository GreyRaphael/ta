use super::{container::Container, statis::Meaner};
use pyo3::prelude::*;

#[pyclass]
pub struct Correlationer {
    x_meaner: Meaner,
    x_sq_meaner: Meaner,
    y_meaner: Meaner,
    y_sq_meaner: Meaner,
    xy_meaner: Meaner,
}

#[pymethods]
impl Correlationer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            x_meaner: Meaner::new(n),
            x_sq_meaner: Meaner::new(n),
            y_meaner: Meaner::new(n),
            y_sq_meaner: Meaner::new(n),
            xy_meaner: Meaner::new(n),
        }
    }

    pub fn update(&mut self, x: f64, y: f64) -> f64 {
        let mean_x = self.x_meaner.update(x);
        let mean_x_sq = self.x_sq_meaner.update(x * x);
        let mean_y = self.y_meaner.update(y);
        let mean_y_sq = self.y_sq_meaner.update(y * y);
        let mean_xy = self.xy_meaner.update(x * y);

        let sigmax_sq = (mean_x_sq - mean_x.powi(2)).sqrt();
        let sigmay_sq = (mean_y_sq - mean_y.powi(2)).sqrt();

        (mean_xy - mean_x * mean_y) / (sigmax_sq * sigmay_sq)
    }
}

#[pyclass]
pub struct Beta {
    x_meaner: Meaner,
    x_sq_meaner: Meaner,
    y_meaner: Meaner,
    xy_meaner: Meaner,
}

#[pymethods]
impl Beta {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            x_meaner: Meaner::new(n),
            x_sq_meaner: Meaner::new(n),
            y_meaner: Meaner::new(n),
            xy_meaner: Meaner::new(n),
        }
    }

    pub fn update(&mut self, x: f64, y: f64) -> f64 {
        let mean_x = self.x_meaner.update(x);
        let mean_x_sq = self.x_sq_meaner.update(x * x);
        let mean_y = self.y_meaner.update(y);
        let mean_xy = self.xy_meaner.update(x * y);

        (mean_xy - mean_x * mean_y) / (mean_x_sq - mean_x.powi(2))
    }
}

#[pyclass]
pub struct TSF {
    n: usize,
    y_meaner: Meaner,
    x_sq_meaner: f64,
    mean_x: f64,
}

#[pymethods]
impl TSF {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            y_meaner: Meaner::new(n),
            x_sq_meaner: ((n + 1) * (2 * n + 1)) as f64 / 6.0, // (1^2+2^2+3^2+n^2)/n
            mean_x: ((1 + n) * n) as f64 / 2.0,                // (1+n)*n/2
        }
    }

    pub fn update(&mut self, y: f64) -> f64 {
        let mean_y = self.y_meaner.update(y);
        let sum_xy: f64 = self
            .y_meaner
            .iter()
            .enumerate()
            .map(|(i, &val)| (i as f64 + 1.0) * val)
            .sum();
        let slope = (sum_xy / self.n as f64 - self.mean_x * mean_y)
            / (self.x_sq_meaner - self.mean_x.powi(2));
        let intercept = mean_y - slope * self.mean_x;

        slope * (self.n + 1) as f64 + intercept // forecast n+1
    }
}

#[pyclass]
pub struct LinearReg {
    n: usize,
    y_meaner: Meaner,
    x_sq_meaner: f64,
    mean_x: f64,
}

#[pymethods]
impl LinearReg {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            y_meaner: Meaner::new(n),
            x_sq_meaner: ((n + 1) * (2 * n + 1)) as f64 / 6.0, // (1^2+2^2+3^2+n^2)/n
            mean_x: ((1 + n) * n) as f64 / 2.0,                // (1+n)*n/2
        }
    }

    pub fn update(&mut self, y: f64) -> (f64, f64, f64, f64) {
        let mean_y = self.y_meaner.update(y);
        let sum_xy: f64 = self
            .y_meaner
            .iter()
            .enumerate()
            .map(|(i, &val)| (i as f64 + 1.0) * val)
            .sum();
        let slope = (sum_xy / self.n as f64 - self.mean_x * mean_y)
            / (self.x_sq_meaner - self.mean_x.powi(2));
        let intercept = mean_y - slope * self.mean_x;

        let last_reg = slope * self.n as f64 + intercept;
        let angle = slope.atan();

        (slope, intercept, last_reg, angle)
    }
}
