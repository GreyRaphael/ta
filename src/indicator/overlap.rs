use crate::{rolling, utils::is_nan_or_inf};
use pyo3::prelude::*;
use std::f64::NAN;

// EMA - Exponential Moving Average
// NOTE: The EMA function has an unstable period.
// Cumulative Exponential Moving Average (EMA) over all data points.
// Since EMA is already cumulative in nature, we can use it as is.
// The first EMA value is typically calculated using the SMA of the first 𝑛 periods in practice
// Because the EMA gives more weight to recent prices, it can react more quickly to price changes compared to the SMA. While this is beneficial for capturing trends early
// Also, because it can fluctuate more in response to short-term price movements, leading to potential whipsaws or false signals.
// real = EMA(real, timeperiod=30)
#[pyclass]
pub struct EMA {
    alpha: f64,
    ema: Option<f64>,
}

#[pymethods]
impl EMA {
    #[new]
    pub fn new(n: usize) -> Self {
        let alpha = 2.0 / (n as f64 + 1.0);
        Self { alpha, ema: None }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        if !is_nan_or_inf(new_val) {
            if let Some(prev_ema) = self.ema {
                self.ema = Some(prev_ema * (1.0 - self.alpha) + new_val * self.alpha);
            } else {
                self.ema = Some(new_val);
            }
        }
        self.ema.unwrap_or(NAN)
    }
}

// SMA - Simple Moving Average
// real = SMA(real, timeperiod=30)
#[pyclass]
pub struct SMA {
    meaner: rolling::statis::Meaner,
}

#[pymethods]
impl SMA {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            meaner: rolling::statis::Meaner::new(timeperiod),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.meaner.update(new_val)
    }
}
