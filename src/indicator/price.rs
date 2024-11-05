use pyo3::prelude::*;

#[pyclass]
pub struct AvgPrice {}

#[pymethods]
impl AvgPrice {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, open: f64, high: f64, low: f64, close: f64) -> f64 {
        (open + high + low + close) / 4.0
    }
}

#[pyclass]
pub struct MedPrice {}

#[pymethods]
impl MedPrice {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, high: f64, low: f64) -> f64 {
        (high + low) / 2.0
    }
}

#[pyclass]
pub struct TypicalPrice {}

#[pymethods]
impl TypicalPrice {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> f64 {
        (high + low + close) / 3.0
    }
}

#[pyclass]
pub struct WeightedClose {}

#[pymethods]
impl WeightedClose {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, high: f64, low: f64, close: f64) -> f64 {
        (high + low + 2.0 * close) / 4.0
    }
}
