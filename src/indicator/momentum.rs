use super::overlap::{EMA, SMA};
use crate::{max, rolling};
use pyo3::prelude::*;

// Awesome Oscillator: SMA5-SMA34
// compare with 0
#[pyclass]
pub struct AO {
    fast_smaer: SMA,
    slow_smaer: SMA,
}

#[pymethods]
impl AO {
    #[new]
    pub fn new(fast_period: usize, slow_period: usize) -> Self {
        Self {
            fast_smaer: SMA::new(fast_period),
            slow_smaer: SMA::new(slow_period),
        }
    }

    pub fn update(&mut self, high: f64, low: f64) -> f64 {
        let med_price = (high + low) / 2.0;
        self.fast_smaer.update(med_price) - self.slow_smaer.update(med_price)
    }
}

// Absolute Price Oscillator: SMA12-SMA26
// compare with 0
#[pyclass]
pub struct APO {
    fast_smaer: SMA,
    slow_smaer: SMA,
}

#[pymethods]
impl APO {
    #[new]
    pub fn new(fast_period: usize, slow_period: usize) -> Self {
        Self {
            fast_smaer: SMA::new(fast_period),
            slow_smaer: SMA::new(slow_period),
        }
    }

    pub fn update(&mut self, price: f64) -> f64 {
        self.fast_smaer.update(price) - self.slow_smaer.update(price)
    }
}

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

#[pyclass]
pub struct Aroon {
    high_maxidxer: rolling::minmax::MaxIndexer,
    low_minidxer: rolling::minmax::MinIndexer,
    period: usize,
}

#[pymethods]
impl Aroon {
    #[new]
    pub fn new(period: usize) -> Self {
        Self {
            high_maxidxer: rolling::minmax::MaxIndexer::new(period),
            low_minidxer: rolling::minmax::MinIndexer::new(period),
            period,
        }
    }

    pub fn update(&mut self, high: f64, low: f64) -> (f64, f64) {
        let (max_high_idx, _) = self.high_maxidxer.update(high);
        let (min_low_idx, _) = self.low_minidxer.update(low);

        let aroon_up = max_high_idx as f64 / self.period as f64;
        let aroon_down = min_low_idx as f64 / self.period as f64;

        (aroon_up, aroon_down)
    }
}

#[pyclass]
pub struct AroonOsc {
    aroon: Aroon,
}

#[pymethods]
impl AroonOsc {
    #[new]
    pub fn new(period: usize) -> Self {
        Self {
            aroon: Aroon::new(period),
        }
    }

    pub fn update(&mut self, high: f64, low: f64) -> f64 {
        let (aroon_up, aroon_down) = self.aroon.update(high, low);

        aroon_up - aroon_down
    }
}

#[pyclass]
pub struct BOP {}

#[pymethods]
impl BOP {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        (close - open) / (high - low)
    }
}

#[pyclass]
pub struct CCI {
    tp_meaner: rolling::statis::Meaner,
    tp_meandever: rolling::statis::Meaner,
}

#[pymethods]
impl CCI {
    #[new]
    pub fn new(period: usize) -> Self {
        Self {
            tp_meaner: rolling::statis::Meaner::new(period),
            tp_meandever: rolling::statis::Meaner::new(period),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> f64 {
        let tp = (high + low + close) / 3.0;
        let tp_avg = self.tp_meaner.update(tp);
        let tp_meandev = self.tp_meandever.update((tp - tp_avg).abs());

        (tp - tp_avg) / (0.015 * tp_meandev)
    }
}

#[pyclass]
pub struct CMO {
    close_deltaer: rolling::delta::Deltaer,
    gain_sumer: rolling::statis::Sumer,
    loss_sumer: rolling::statis::Sumer,
}

#[pymethods]
impl CMO {
    #[new]
    pub fn new(period: usize) -> Self {
        Self {
            close_deltaer: rolling::delta::Deltaer::new(2),
            gain_sumer: rolling::statis::Sumer::new(period),
            loss_sumer: rolling::statis::Sumer::new(period),
        }
    }

    pub fn update(&mut self, close: f64) -> f64 {
        let close_delta = self.close_deltaer.update(close);
        let up;
        let down;
        if close_delta > 0.0 {
            up = self.gain_sumer.update(close_delta);
            down = self.loss_sumer.update(0.0);
        } else {
            up = self.gain_sumer.update(0.0);
            down = self.loss_sumer.update(close_delta.abs());
        }

        (up - down) / (up + down)
    }
}

#[pyclass]
pub struct DX {
    plus_dier: PlusDI,
    minus_dier: MinusDI,
}

#[pymethods]
impl DX {
    #[new]
    pub fn new(period: usize) -> Self {
        Self {
            plus_dier: PlusDI::new(period),
            minus_dier: MinusDI::new(period),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> f64 {
        let plus_di = self.plus_dier.update(high, low, close);
        let minus_di = self.minus_dier.update(high, low, close);

        (plus_di - minus_di).abs() / (plus_di + minus_di)
    }
}

#[pyclass]
pub struct KDJ {
    lowest: rolling::minmax::Miner,
    highest: rolling::minmax::Maxer,
    d_liner: rolling::statis::Meaner,
    j_liner: rolling::statis::Meaner,
}

#[pymethods]
impl KDJ {
    #[new]
    pub fn new(view_period: usize, d_period: usize, j_period: usize) -> Self {
        Self {
            lowest: rolling::minmax::Miner::new(view_period),
            highest: rolling::minmax::Maxer::new(view_period),
            d_liner: rolling::statis::Meaner::new(d_period),
            j_liner: rolling::statis::Meaner::new(j_period),
        }
    }

    pub fn update(&mut self, price: f64, high: f64, low: f64) -> (f64, f64, f64) {
        let lowest_low = self.lowest.update(low);
        let highest_high = self.highest.update(high);
        let k_line = (price - lowest_low) / (highest_high - lowest_low);
        let d_line = self.d_liner.update(k_line);
        let j_line_raw = 3.0 * k_line - 2.0 * d_line;
        let j_line = self.j_liner.update(j_line_raw);

        (k_line, d_line, j_line)
    }
}

// KST Oscillator: ROC10 + 2*ROC15 + 3*ROC20 + 4*ROC30
#[pyclass]
pub struct KST {
    ptchanger: rolling::delta::Pctchanger,
    max_period: usize,
}

#[pymethods]
impl KST {
    #[new]
    pub fn new(max_period: usize) -> Self {
        Self {
            ptchanger: rolling::delta::Pctchanger::new(max_period),
            max_period,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.ptchanger.update(new_val);

        let roc10 = self.ptchanger.partial(self.max_period - 9, self.max_period);
        let roc15 = self
            .ptchanger
            .partial(self.max_period - 14, self.max_period);
        let roc20 = self
            .ptchanger
            .partial(self.max_period - 19, self.max_period);
        let roc30 = self
            .ptchanger
            .partial(self.max_period - 29, self.max_period);

        roc10 + 2.0 * roc15 + 3.0 * roc20 + 4.0 * roc30
    }
}

#[pyclass]
pub struct MACD {
    fast_ema: EMA,
    slow_ema: EMA,
    signal_ema: EMA,
}

#[pymethods]
impl MACD {
    #[new]
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Self {
            fast_ema: EMA::new(fast_period),
            slow_ema: EMA::new(slow_period),
            signal_ema: EMA::new(signal_period),
        }
    }

    pub fn update(&mut self, new_val: f64) -> (f64, f64, f64) {
        // velocity(DIF), 1st derivative
        let macd_line = self.fast_ema.update(new_val) - self.slow_ema.update(new_val);
        // DEA(Difference Exponential Average)
        let signal_line = self.signal_ema.update(macd_line);
        // acceleration, 2nd derivative
        let macd_hist = macd_line - signal_line;

        (macd_line, signal_line, macd_hist)
    }
}

#[pyclass]
pub struct MinusDM {
    high_vec: rolling::container::Container,
    low_vec: rolling::container::Container,
    minus_dm_sumer: rolling::statis::Sumer,
}

#[pymethods]
impl MinusDM {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            high_vec: rolling::container::Container::new(2),
            low_vec: rolling::container::Container::new(2),
            minus_dm_sumer: rolling::statis::Sumer::new(timeperiod),
        }
    }

    pub fn update(&mut self, high: f64, low: f64) -> f64 {
        let (pre_high, _) = self.high_vec.update(high);
        let (pre_low, _) = self.low_vec.update(low);

        let high_diff = high - pre_high;
        let low_diff_reverse = pre_low - low;

        let minus_dm: f64;
        if (low_diff_reverse > high_diff) && (low_diff_reverse > 0.0) {
            minus_dm = low_diff_reverse;
        } else {
            minus_dm = 0.0;
        }

        self.minus_dm_sumer.update(minus_dm)
    }
}

#[pyclass]
pub struct PlusDM {
    high_vec: rolling::container::Container,
    low_vec: rolling::container::Container,
    plus_dm_sumer: rolling::statis::Sumer,
}

#[pymethods]
impl PlusDM {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            high_vec: rolling::container::Container::new(2),
            low_vec: rolling::container::Container::new(2),
            plus_dm_sumer: rolling::statis::Sumer::new(timeperiod),
        }
    }

    pub fn update(&mut self, high: f64, low: f64) -> f64 {
        let (pre_high, _) = self.high_vec.update(high);
        let (pre_low, _) = self.low_vec.update(low);

        let high_diff = high - pre_high;
        let low_diff_reverse = pre_low - low;

        let plus_dm: f64;
        if (high_diff > low_diff_reverse) && (high_diff > 0.0) {
            plus_dm = high_diff;
        } else {
            plus_dm = 0.0;
        }

        self.plus_dm_sumer.update(plus_dm)
    }
}

#[pyclass]
pub struct PlusDI {
    plus_dmer: PlusDM,
    tr_sumer: rolling::statis::Sumer,
    close_container: rolling::container::Container,
}

#[pymethods]
impl PlusDI {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            plus_dmer: PlusDM::new(timeperiod),
            tr_sumer: rolling::statis::Sumer::new(timeperiod),
            close_container: rolling::container::Container::new(2),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> f64 {
        let (preclose, _) = self.close_container.update(close);
        let smoothed_plus_dm = self.plus_dmer.update(high, low);
        let tr = max!(high - low, (high - preclose).abs(), (low - preclose).abs());
        let smoothed_tr = self.tr_sumer.update(tr);

        100.0 * smoothed_plus_dm / smoothed_tr
    }
}

#[pyclass]
pub struct MinusDI {
    minus_dmer: MinusDM,
    tr_sumer: rolling::statis::Sumer,
    close_container: rolling::container::Container,
}

#[pymethods]
impl MinusDI {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            minus_dmer: MinusDM::new(timeperiod),
            tr_sumer: rolling::statis::Sumer::new(timeperiod),
            close_container: rolling::container::Container::new(2),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> f64 {
        let (preclose, _) = self.close_container.update(close);
        let smoothed_minus_dm = self.minus_dmer.update(high, low);
        let tr = max!(high - low, (high - preclose).abs(), (low - preclose).abs());
        let smoothed_tr = self.tr_sumer.update(tr);

        100.0 * smoothed_minus_dm / smoothed_tr
    }
}

#[pyclass]
pub struct MFI {
    pos_mfer: rolling::statis::Sumer,
    neg_mfer: rolling::statis::Sumer,
    tp_deltaer: rolling::delta::Deltaer,
}

#[pymethods]
impl MFI {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            pos_mfer: rolling::statis::Sumer::new(timeperiod),
            neg_mfer: rolling::statis::Sumer::new(timeperiod),
            tp_deltaer: rolling::delta::Deltaer::new(2),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64, volume: f64) -> f64 {
        let typical_price = (high + low + close) / 3.0;
        let tp_diff = self.tp_deltaer.update(typical_price);
        let pos_mf;
        let neg_mf;
        if tp_diff > 0.0 {
            pos_mf = self.pos_mfer.update(typical_price * volume);
            neg_mf = self.neg_mfer.update(0.0);
        } else {
            pos_mf = self.pos_mfer.update(0.0);
            neg_mf = self.neg_mfer.update(typical_price * volume);
        }
        let mfr = pos_mf / neg_mf;

        mfr / (mfr + 1.0)
    }
}

#[pyclass]
pub struct MOM {
    deltaer: rolling::delta::Deltaer,
}

#[pymethods]
impl MOM {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            deltaer: rolling::delta::Deltaer::new(timeperiod),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.deltaer.update(new_val)
    }
}

#[pyclass]
pub struct PPO {
    fast_emaer: EMA,
    slow_emaer: EMA,
}

#[pymethods]
impl PPO {
    #[new]
    pub fn new(fast_period: usize, slow_period: usize) -> Self {
        Self {
            fast_emaer: EMA::new(fast_period),
            slow_emaer: EMA::new(slow_period),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.fast_emaer.update(new_val) / self.slow_emaer.update(new_val) - 1.0
    }
}

#[pyclass]
pub struct PPOHist {
    ppo: PPO,
    signal_emaer: EMA,
}

#[pymethods]
impl PPOHist {
    #[new]
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        Self {
            ppo: PPO::new(fast_period, slow_period),
            signal_emaer: EMA::new(signal_period),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let ppo_val = self.ppo.update(new_val);
        let signal_line = self.signal_emaer.update(ppo_val);

        ppo_val - signal_line
    }
}

#[pyclass]
pub struct ROC {
    pctchanger: rolling::delta::Pctchanger,
}

#[pymethods]
impl ROC {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            pctchanger: rolling::delta::Pctchanger::new(timeperiod),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.pctchanger.update(new_val)
    }
}

#[pyclass]
pub struct ROCR {
    container: rolling::container::Container,
}

#[pymethods]
impl ROCR {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            container: rolling::container::Container::new(timeperiod),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.container.update(new_val);

        self.container.tail() / self.container.head()
    }
}

#[pyclass]
pub struct RSI {
    price_deltaer: rolling::delta::Deltaer,
    up_moves: rolling::statis::Sumer,
    down_moves: rolling::statis::Sumer,
    up_smoother: EMA,
    down_smoother: EMA,
}

#[pymethods]
impl RSI {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            price_deltaer: rolling::delta::Deltaer::new(2),
            up_moves: rolling::statis::Sumer::new(timeperiod),
            down_moves: rolling::statis::Sumer::new(timeperiod),
            up_smoother: EMA::new(timeperiod),
            down_smoother: EMA::new(timeperiod),
        }
    }

    pub fn update(&mut self, price: f64) -> f64 {
        let delta = self.price_deltaer.update(price);
        let up_sum;
        let down_sum;
        if delta > 0.0 {
            up_sum = self.up_moves.update(delta.abs());
            down_sum = self.down_moves.update(0.0);
        } else {
            up_sum = self.up_moves.update(0.0);
            down_sum = self.down_moves.update(delta.abs());
        }

        let rs = self.up_smoother.update(up_sum) / self.down_smoother.update(down_sum);

        rs / (1.0 + rs)
    }
}

#[pyclass]
pub struct Stoch {
    high_maxer: rolling::minmax::Maxer,
    low_miner: rolling::minmax::Miner,
    slowk_liner: SMA,
    slowd_liner: SMA,
}

#[pymethods]
impl Stoch {
    #[new]
    pub fn new(fastk_period: usize, slowk_period: usize, slowd_period: usize) -> Self {
        Self {
            high_maxer: rolling::minmax::Maxer::new(fastk_period),
            low_miner: rolling::minmax::Miner::new(fastk_period),
            slowk_liner: SMA::new(slowk_period),
            slowd_liner: SMA::new(slowd_period),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> (f64, f64) {
        let high_max = self.high_maxer.update(high);
        let low_min = self.low_miner.update(low);
        let fastk_line = (close - low_min) / (high_max - low_min);
        let slowk_line = self.slowk_liner.update(fastk_line);
        let slowd_line = self.slowd_liner.update(slowk_line);

        (slowk_line, slowd_line)
    }
}

#[pyclass]
pub struct StochCHF {
    high_maxer: rolling::minmax::Maxer,
    low_miner: rolling::minmax::Miner,
    fastd_liner: SMA,
}

#[pymethods]
impl StochCHF {
    #[new]
    pub fn new(fastk_period: usize, fastd_period: usize) -> Self {
        Self {
            high_maxer: rolling::minmax::Maxer::new(fastk_period),
            low_miner: rolling::minmax::Miner::new(fastk_period),
            fastd_liner: SMA::new(fastd_period),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> (f64, f64) {
        let high_max = self.high_maxer.update(high);
        let low_min = self.low_miner.update(low);
        let fastk_line = (close - low_min) / (high_max - low_min);
        let fastd_line = self.fastd_liner.update(fastk_line);

        (fastk_line, fastd_line)
    }
}

#[pyclass]
pub struct StochRSI {
    rsier: RSI,
    rsi_minmaxer: rolling::minmax::MinMaxer,
    fastk_liner: SMA,
    fastd_liner: SMA,
}

#[pymethods]
impl StochRSI {
    #[new]
    pub fn new(rsi_period: usize, fastk_period: usize, fastd_period: usize) -> Self {
        Self {
            rsier: RSI::new(rsi_period),
            rsi_minmaxer: rolling::minmax::MinMaxer::new(rsi_period),
            fastk_liner: SMA::new(fastk_period),
            fastd_liner: SMA::new(fastd_period),
        }
    }

    pub fn update(&mut self, new_val: f64) -> (f64, f64) {
        let rsi = self.rsier.update(new_val);
        let (rsi_min, rsi_max) = self.rsi_minmaxer.update(rsi);
        let stoch_rsi = (rsi - rsi_min) / (rsi_max - rsi_min);
        let fastk = self.fastk_liner.update(stoch_rsi);
        let fastd = self.fastd_liner.update(stoch_rsi);

        (fastk, fastd)
    }
}

#[pyclass]
pub struct TRIX {
    ema_lv1: EMA,
    ema_lv2: EMA,
    ema_lv3: EMA,
    ema_lv3_pctc: rolling::delta::Pctchanger,
}

#[pymethods]
impl TRIX {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            ema_lv1: EMA::new(timeperiod),
            ema_lv2: EMA::new(timeperiod),
            ema_lv3: EMA::new(timeperiod),
            ema_lv3_pctc: rolling::delta::Pctchanger::new(2),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let lv1 = self.ema_lv1.update(new_val);
        let lv2 = self.ema_lv2.update(lv1);
        let lv3 = self.ema_lv3.update(lv2);

        self.ema_lv3_pctc.update(lv3)
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

#[pyclass]
pub struct WILLR {
    high_maxer: rolling::minmax::Maxer,
    low_miner: rolling::minmax::Miner,
}

#[pymethods]
impl WILLR {
    #[new]
    pub fn new(timeperiod: usize) -> Self {
        Self {
            high_maxer: rolling::minmax::Maxer::new(timeperiod),
            low_miner: rolling::minmax::Miner::new(timeperiod),
        }
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> f64 {
        let high_max = self.high_maxer.update(high);
        let low_min = self.low_miner.update(low);

        (high_max - close) / (high_max - low_min)
    }
}
