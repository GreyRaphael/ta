use std::f64::NAN;

use pyo3::prelude::*;

// True Range
#[pyclass]
pub struct TR {}

#[pymethods]
impl TR {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, high: f64, low: f64, preclose: f64) -> f64 {
        (high - low)
            .max((high - preclose).abs())
            .max((low - preclose).abs())
    }
}

// Average True Range
#[pyclass]
pub struct ATR {
    n: usize,
    atr: Option<f64>,
    init_trs: Vec<f64>,
}

#[pymethods]
impl ATR {
    #[new]
    pub fn new(period: usize) -> Self {
        Self {
            n: period,
            atr: None,
            init_trs: Vec::with_capacity(period),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, preclose: f64) -> f64 {
        let tr = (high - low)
            .max((high - preclose).abs())
            .max((low - preclose).abs());

        if let Some(prev_atr) = self.atr {
            self.atr = Some((prev_atr * (self.n - 1) as f64 + tr) / self.n as f64);
        } else {
            self.init_trs.push(tr);
            if self.init_trs.len() == self.n {
                self.atr = Some(self.init_trs.iter().sum::<f64>() / self.n as f64);
            }
        }
        self.atr.unwrap_or(NAN)
    }
}
