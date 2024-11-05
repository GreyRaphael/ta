use crate::rolling;
use pyo3::prelude::*;
use std::f64::NAN;

// True Range
#[pyclass]
pub struct TR {
    close_vec: rolling::container::Container,
}

#[pymethods]
impl TR {
    #[new]
    pub fn new() -> Self {
        Self {
            close_vec: rolling::container::Container::new(2),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> f64 {
        self.close_vec.update(close);
        let preclose = self.close_vec.head();
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
    close_vec: rolling::container::Container,
}

#[pymethods]
impl ATR {
    #[new]
    pub fn new(period: usize) -> Self {
        Self {
            n: period,
            atr: None,
            init_trs: Vec::with_capacity(period),
            close_vec: rolling::container::Container::new(2),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> f64 {
        self.close_vec.update(close);
        let preclose = self.close_vec.head();
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

// NATR - Normalized Average True Range
#[pyclass]
pub struct NATR {
    n: usize,
    atr: Option<f64>,
    init_trs: Vec<f64>,
    close_vec: rolling::container::Container,
}

#[pymethods]
impl NATR {
    #[new]
    pub fn new(period: usize) -> Self {
        Self {
            n: period,
            atr: None,
            init_trs: Vec::with_capacity(period),
            close_vec: rolling::container::Container::new(2),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> f64 {
        self.close_vec.update(close);
        let preclose = self.close_vec.head();
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
        self.atr.unwrap_or(NAN) / close
    }
}
