use crate::rolling;
use pyo3::prelude::*;

// AD - Chaikin A/D Line
#[pyclass]
pub struct AD {
    sumer: rolling::statis::Sumer,
}

#[pymethods]
impl AD {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            sumer: rolling::statis::Sumer::new(timeperiod),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64, volume: usize) -> f64 {
        let mfm = (2.0 * close - low - high) / (high - low);
        let mfv = mfm * volume as f64;

        self.sumer.update(mfv)
    }
}
