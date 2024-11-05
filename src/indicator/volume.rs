use super::overlap::EMA;
use crate::{rolling, utils::sign};
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

// Chaikin A/D Oscillator
#[pyclass]
pub struct ADOSC {
    ema_fast: EMA,
    ema_slow: EMA,
    ader: AD,
}

#[pymethods]
impl ADOSC {
    #[new]
    pub fn new(timeperiod: usize, fastperiod: usize, slowperiod: usize) -> Self {
        Self {
            ema_fast: EMA::new(fastperiod),
            ema_slow: EMA::new(slowperiod),
            ader: AD::new(timeperiod),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64, volume: usize) -> f64 {
        let ad_value = self.ader.update(high, low, close, volume);

        self.ema_fast.update(ad_value) - self.ema_slow.update(ad_value)
    }
}

// OBV - On Balance Volume
#[pyclass]
pub struct OBV {
    obv_sumer: rolling::statis::Sumer,
    close_deltaer: rolling::delta::Deltaer,
}

#[pymethods]
impl OBV {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            obv_sumer: rolling::statis::Sumer::new(timeperiod),
            close_deltaer: rolling::delta::Deltaer::new(2),
        }
    }

    pub fn update(&mut self, close: f64, volume: usize) -> f64 {
        let signed_volume = sign(self.close_deltaer.update(close)) as f64 * volume as f64;

        self.obv_sumer.update(signed_volume)
    }
}
