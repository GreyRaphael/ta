use ordered_float::OrderedFloat;
use pyo3::prelude::*;
use std::collections::BTreeMap;
use std::f64::NAN;
use std::ops::Bound::{Excluded, Unbounded};

use crate::utils::is_nan_or_inf;

#[pyclass]
pub struct Quantiler {
    dataset: BTreeMap<OrderedFloat<f64>, usize>,
    buf: Vec<f64>,
    n: usize,
    cur_idx: usize,
    nan_count: usize,
    quantile: f64,
}

#[pymethods]
impl Quantiler {
    #[new]
    pub fn new(n: usize, quantile: f64) -> Self {
        Self {
            dataset: BTreeMap::new(),
            buf: vec![NAN; n],
            n,
            cur_idx: 0,
            nan_count: n,
            quantile,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        let old_val = self.buf[self.cur_idx];
        self.buf[self.cur_idx] = new_val;
        self.cur_idx = (self.cur_idx + 1) % self.n;

        // Update nan_count and dataset based on new_val
        if is_nan_or_inf(new_val) {
            self.nan_count += 1;
        } else {
            let ordered_val = OrderedFloat(new_val);
            *self.dataset.entry(ordered_val).or_insert(0) += 1;
        }

        // Update nan_count and dataset based on old_val
        if is_nan_or_inf(old_val) {
            self.nan_count -= 1;
        } else {
            let ordered_old_val = OrderedFloat(old_val);
            if let Some(count) = self.dataset.get_mut(&ordered_old_val) {
                *count -= 1;
                if *count == 0 {
                    self.dataset.remove(&ordered_old_val);
                }
            }
        }

        if self.nan_count > 0 {
            NAN
        } else {
            let size: usize = self.dataset.iter().map(|(_, &count)| count).sum();

            let index = (size as f64 - 1.0) * self.quantile;
            let lower_index = index.floor() as usize;
            let fraction = index - lower_index as f64;

            // Iterate through the sorted dataset to find the lower and upper values
            let mut cumulative = 0;
            let mut lower_value = NAN;
            let mut upper_value = NAN;

            for (&key, &count) in &self.dataset {
                if cumulative + count > lower_index {
                    lower_value = key.0;
                    if fraction == 0.0 {
                        upper_value = key.0;
                        break;
                    } else {
                        // Find the next value
                        if let Some((&next_key, _)) =
                            self.dataset.range((Excluded(&key), Unbounded)).next()
                        {
                            upper_value = next_key.0;
                        } else {
                            upper_value = key.0;
                        }
                        break;
                    }
                }
                cumulative += count;
            }

            if lower_value.is_nan() || upper_value.is_nan() {
                NAN
            } else {
                lower_value + fraction * (upper_value - lower_value)
            }
        }
    }
}
