use pyo3::prelude::*;

#[pyclass]
pub struct EMAer {
    alpha: f64,
    result: Option<f64>,
}

#[pymethods]
impl EMAer {
    #[new]
    pub fn new(n: usize) -> Self {
        let alpha = 2.0 / (n as f64 + 1.0);
        Self {
            alpha,
            result: None,
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        match self.result {
            Some(ref mut current_ema) => {
                *current_ema = *current_ema * (1.0 - self.alpha) + new_val * self.alpha;
                *current_ema
            }
            None => {
                self.result = Some(new_val);
                new_val
            }
        }
    }
}
