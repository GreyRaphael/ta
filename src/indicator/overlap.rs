use crate::{rolling, utils::is_nan_or_inf};
use pyo3::prelude::*;
use std::f64::NAN;

// DEMA - Double Exponential Moving Average
// EMA1 = EMA of price
// EMA2 = EMA of EMA1
// DEMA = (2 x EMA1) - EMA2
#[pyclass]
pub struct DEMA {
    ema_lv1: EMA,
    ema_lv2: EMA,
}

#[pymethods]
impl DEMA {
    #[new]
    pub fn new(period: usize) -> Self {
        Self {
            ema_lv1: EMA::new(period),
            ema_lv2: EMA::new(period),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let lv1 = self.ema_lv1.update(new_val);
        let lv2 = self.ema_lv2.update(lv1);

        2.0 * lv1 - lv2
    }
}

// EMA - Exponential Moving Average
// NOTE: The EMA function has an unstable period.
// Cumulative Exponential Moving Average (EMA) over all data points.
// Since EMA is already cumulative in nature, we can use it as is.
// The first EMA value is typically calculated using the SMA of the first 𝑛 periods in practice
// Because the EMA gives more weight to recent prices, it can react more quickly to price changes compared to the SMA. While this is beneficial for capturing trends early
// Also, because it can fluctuate more in response to short-term price movements, leading to potential whipsaws or false signals.
// EMA = prev_EMA x (1 – SmoothingFactor) + Price x SmoothingFactor
// SmoothingFactor = 2 / (period + 1)
#[pyclass]
pub struct EMA {
    alpha: f64,
    ema: Option<f64>,
}

#[pymethods]
impl EMA {
    #[new]
    pub fn new(period: usize) -> Self {
        let alpha = 2.0 / (period as f64 + 1.0);
        Self { alpha, ema: None }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        if is_nan_or_inf(new_val) {
            NAN
        } else {
            if let Some(prev_ema) = self.ema {
                self.ema = Some(prev_ema * (1.0 - self.alpha) + new_val * self.alpha);
            } else {
                self.ema = Some(new_val);
            }
            self.ema.unwrap()
        }
    }
}

// KAMA - Kaufman Adaptive Moving Average
// NOTE: The KAMA function has an unstable period.
#[pyclass]
pub struct KAMA {
    price_vec: rolling::container::Container,
    volatility_sumer: rolling::statis::Sumer,
    fast_sc: f64,
    slow_sc: f64,
    kama: Option<f64>,
}

#[pymethods]
impl KAMA {
    #[new]
    pub fn new(er_period: usize, fast_period: usize, slow_period: usize) -> Self {
        Self {
            price_vec: rolling::container::Container::new(er_period), // typical 10
            volatility_sumer: rolling::statis::Sumer::new(er_period),
            fast_sc: 2.0 / (fast_period as f64 + 1.0), // typical 2
            slow_sc: 2.0 / (slow_period as f64 + 1.0), // typical 30
            kama: None,
        }
    }

    pub fn update(&mut self, price: f64, preprice: f64) -> f64 {
        if is_nan_or_inf(price) {
            NAN
        } else {
            self.price_vec.update(price);
            let change = (price - self.price_vec.head()).abs();
            let volatility = self.volatility_sumer.update((price - preprice).abs());
            let er = change / volatility;
            let sc = (er * (self.fast_sc - self.slow_sc) + self.slow_sc).powi(2);

            if let Some(prev_kama) = self.kama {
                self.kama = Some(prev_kama + sc * (price - prev_kama));
            } else {
                self.kama = Some(price);
            }

            self.kama.unwrap()
        }
    }
}

// MAMA - MESA Adaptive Moving Average, [TO DIFFICULT, must use hilbert_transform crate]
// NOTE: The MAMA function has an unstable period.
// mama, fama = MAMA(real, fastlimit=0, slowlimit=0)
#[pyclass]
pub struct MAMA {}

#[pymethods]
impl MAMA {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {}
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        1.0
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

// TEMA: Triple Exponential Moving Average
// NOTE: The TEMA function has an unstable period, different from talib.T3
#[pyclass]
pub struct TEMA {
    ema_lv1: EMA,
    ema_lv2: EMA,
    ema_lv3: EMA,
}

#[pymethods]
impl TEMA {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            ema_lv1: EMA::new(timeperiod),
            ema_lv2: EMA::new(timeperiod),
            ema_lv3: EMA::new(timeperiod),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let lv1 = self.ema_lv1.update(new_val);
        let lv2 = self.ema_lv2.update(lv1);
        let lv3 = self.ema_lv3.update(lv2);

        3.0 * lv1 - 3.0 * lv2 + lv3
    }
}
