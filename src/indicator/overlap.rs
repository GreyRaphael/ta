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

// T3 - Triple Exponential Moving Average (T3)
// NOTE: The T3 function has an unstable period, typical vfactor is 0.7
#[pyclass]
pub struct T3 {
    ema_lv1: EMA,
    ema_lv2: EMA,
    ema_lv3: EMA,
    ema_lv4: EMA,
    ema_lv5: EMA,
    ema_lv6: EMA,
    c1: f64,
    c2: f64,
    c3: f64,
    c4: f64,
}

#[pymethods]
impl T3 {
    #[new]
    pub fn new(timeperiod: usize, vfactor: f64) -> Self {
        Self {
            ema_lv1: EMA::new(timeperiod),
            ema_lv2: EMA::new(timeperiod),
            ema_lv3: EMA::new(timeperiod),
            ema_lv4: EMA::new(timeperiod),
            ema_lv5: EMA::new(timeperiod),
            ema_lv6: EMA::new(timeperiod),
            c1: -vfactor.powi(3),
            c2: 3.0 * vfactor.powi(2) + 3.0 * vfactor.powi(3),
            c3: -6.0 * vfactor.powi(2) - 3.0 * vfactor - 3.0 * vfactor.powi(3),
            c4: 1.0 + 3.0 * vfactor + 3.0 * vfactor.powi(2) + vfactor.powi(3),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let lv1 = self.ema_lv1.update(new_val);
        let lv2 = self.ema_lv2.update(lv1);
        let lv3 = self.ema_lv3.update(lv2);
        let lv4 = self.ema_lv4.update(lv3);
        let lv5 = self.ema_lv5.update(lv4);
        let lv6 = self.ema_lv6.update(lv5);

        self.c1 * lv6 + self.c2 * lv5 + self.c3 * lv4 + self.c4 * lv3
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

// TRIMA - Triangular Moving Average
// SMA(SMA(timeperiod))
#[pyclass]
pub struct TRIMA {
    sma1: SMA,
    sma2: SMA,
}

#[pymethods]
impl TRIMA {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            sma1: SMA::new(timeperiod),
            sma2: SMA::new(timeperiod),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.sma2.update(self.sma1.update(new_val))
    }
}

// WMA - Weighted Moving Average
#[pyclass]
pub struct WMA {
    container: rolling::container::Container,
    weights: Vec<f64>,
}

#[pymethods]
impl WMA {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        let weight_sum = (timeperiod * (timeperiod + 1)) as f64 / 2.0;
        Self {
            container: rolling::container::Container::new(timeperiod),
            weights: (0..timeperiod)
                .map(|i| (i as f64 + 1.0) / weight_sum)
                .collect::<Vec<_>>(),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.container.update(new_val);
        self.container
            .iter()
            .zip(&self.weights)
            .map(|(x, w)| x * w)
            .sum()
    }
}
