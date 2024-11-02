use crate::rolling;
use pyo3::prelude::*;

// https://github.com/TA-Lib/ta-lib-python/blob/master/docs/func_groups/momentum_indicators.md

// real = ULTOSC(high, low, close, timeperiod1=7, timeperiod2=14, timeperiod3=28)

#[pyclass]
pub struct ULTOSC {
    timeperiod1_bp_sumer: rolling::sum::Sumer,
    timeperiod2_bp_sumer: rolling::sum::Sumer,
    timeperiod3_bp_sumer: rolling::sum::Sumer,
    timeperiod1_tr_sumer: rolling::sum::Sumer,
    timeperiod2_tr_sumer: rolling::sum::Sumer,
    timeperiod3_tr_sumer: rolling::sum::Sumer,
}

#[pymethods]
impl ULTOSC {
    #[new]
    pub fn new(timeperiod1: usize, timeperiod2: usize, timeperiod3: usize) -> Self {
        Self {
            timeperiod1_bp_sumer: rolling::sum::Sumer::new(timeperiod1),
            timeperiod2_bp_sumer: rolling::sum::Sumer::new(timeperiod2),
            timeperiod3_bp_sumer: rolling::sum::Sumer::new(timeperiod3),
            timeperiod1_tr_sumer: rolling::sum::Sumer::new(timeperiod1),
            timeperiod2_tr_sumer: rolling::sum::Sumer::new(timeperiod2),
            timeperiod3_tr_sumer: rolling::sum::Sumer::new(timeperiod3),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64, preclose: f64) -> f64 {
        let bp = close - low.min(preclose);
        let tr = high.max(preclose) - low.min(preclose);

        let timeperiod1_bp_avg =
            self.timeperiod1_bp_sumer.update(bp) / self.timeperiod1_tr_sumer.update(tr);
        let timeperiod2_bp_avg =
            self.timeperiod2_bp_sumer.update(bp) / self.timeperiod2_tr_sumer.update(tr);
        let timeperiod3_bp_avg =
            self.timeperiod3_bp_sumer.update(bp) / self.timeperiod3_tr_sumer.update(tr);

        100.0 * (4.0 * timeperiod1_bp_avg + 2.0 * timeperiod2_bp_avg + timeperiod3_bp_avg) / 7.0
    }
}
