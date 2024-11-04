use crate::{max, rolling};
use pyo3::prelude::*;

// https://github.com/TA-Lib/ta-lib-python/blob/master/docs/func_groups/momentum_indicators.md

// real = ADX(high, low, close, timeperiod=14)

#[pyclass]
pub struct ADX {
    high_vec: rolling::container::Container,
    low_vec: rolling::container::Container,
    plus_dm_sumer: rolling::statis::Sumer,
    minus_dm_sumer: rolling::statis::Sumer,
    tr_sumer: rolling::statis::Sumer,
    dx_meaner: rolling::statis::Meaner,
}

#[pymethods]
impl ADX {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            high_vec: rolling::container::Container::new(2),
            low_vec: rolling::container::Container::new(2),
            plus_dm_sumer: rolling::statis::Sumer::new(timeperiod),
            minus_dm_sumer: rolling::statis::Sumer::new(timeperiod),
            tr_sumer: rolling::statis::Sumer::new(timeperiod),
            dx_meaner: rolling::statis::Meaner::new(timeperiod),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, preclose: f64) -> f64 {
        self.high_vec.update(high);
        let pre_high = self.high_vec.head();
        self.low_vec.update(low);
        let pre_low = self.low_vec.head();

        let high_diff = high - pre_high;
        let low_diff_reverse = pre_low - low;

        let plus_dm: f64;
        if (high_diff > low_diff_reverse) && (high_diff > 0.0) {
            plus_dm = high_diff;
        } else {
            plus_dm = 0.0;
        }
        let minus_dm: f64;
        if (low_diff_reverse > high_diff) && (low_diff_reverse > 0.0) {
            minus_dm = low_diff_reverse;
        } else {
            minus_dm = 0.0;
        }
        let tr = max!(high - low, (high - preclose).abs(), (low - preclose).abs());

        let smoothed_plus_dm = self.plus_dm_sumer.update(plus_dm);
        let smoothed_minus_dm = self.minus_dm_sumer.update(minus_dm);
        let smoothed_tr = self.tr_sumer.update(tr);

        let di_plus = 100.0 * smoothed_plus_dm / smoothed_tr;
        let di_minus = 100.0 * smoothed_minus_dm / smoothed_tr;
        let dx = 100.0 * (di_plus - di_minus).abs() / (di_plus + di_minus);
        let adx = self.dx_meaner.update(dx);

        adx
    }
}

// real = ADXR(high, low, close, timeperiod=14)

#[pyclass]
pub struct ADXR {
    adxer: ADX,
    adx_container: rolling::container::Container,
}

#[pymethods]
impl ADXR {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            adxer: ADX::new(timeperiod),
            adx_container: rolling::container::Container::new(timeperiod),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, preclose: f64) -> f64 {
        let adx = self.adxer.update(high, low, preclose);
        self.adx_container.update(adx);
        let tail_adx = self.adx_container.tail();
        let head_adx = self.adx_container.head();

        (tail_adx + head_adx) / 2.0
    }
}

// real = ULTOSC(high, low, close, timeperiod1=7, timeperiod2=14, timeperiod3=28)

#[pyclass]
pub struct ULTOSC {
    timeperiod1_bp_sumer: rolling::statis::Sumer,
    timeperiod2_bp_sumer: rolling::statis::Sumer,
    timeperiod3_bp_sumer: rolling::statis::Sumer,
    timeperiod1_tr_sumer: rolling::statis::Sumer,
    timeperiod2_tr_sumer: rolling::statis::Sumer,
    timeperiod3_tr_sumer: rolling::statis::Sumer,
}

#[pymethods]
impl ULTOSC {
    #[new]
    pub fn new(timeperiod1: usize, timeperiod2: usize, timeperiod3: usize) -> Self {
        Self {
            timeperiod1_bp_sumer: rolling::statis::Sumer::new(timeperiod1),
            timeperiod2_bp_sumer: rolling::statis::Sumer::new(timeperiod2),
            timeperiod3_bp_sumer: rolling::statis::Sumer::new(timeperiod3),
            timeperiod1_tr_sumer: rolling::statis::Sumer::new(timeperiod1),
            timeperiod2_tr_sumer: rolling::statis::Sumer::new(timeperiod2),
            timeperiod3_tr_sumer: rolling::statis::Sumer::new(timeperiod3),
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
