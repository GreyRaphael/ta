use super::container::Container;
use pyo3::prelude::*;

#[pyclass]
pub struct Deltaer {
    container: Container,
}

#[pymethods]
impl Deltaer {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            container: Container::new(n),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.container.update(new_val);

        self.container.tail() - self.container.head()
    }

    pub fn get(&self, idx: usize) -> f64 {
        self.container.get(idx)
    }

    pub fn head(&self) -> f64 {
        self.container.head()
    }

    pub fn tail(&self) -> f64 {
        self.container.tail()
    }

    pub fn partial(&self, start: usize, end: usize) -> f64 {
        self.get(end) - self.get(start)
    }
}

#[pyclass]
pub struct Pctchanger {
    container: Container,
}

#[pymethods]
impl Pctchanger {
    #[new]
    pub fn new(n: usize) -> Self {
        Self {
            container: Container::new(n),
        }
    }

    pub fn update(&mut self, new_val: f64) -> f64 {
        self.container.update(new_val);

        self.container.tail() / self.container.head() - 1.0
    }

    pub fn get(&self, idx: usize) -> f64 {
        self.container.get(idx)
    }

    pub fn head(&self) -> f64 {
        self.container.head()
    }

    pub fn tail(&self) -> f64 {
        self.container.tail()
    }

    pub fn partial(&self, start: usize, end: usize) -> f64 {
        self.get(end) / self.get(start) - 1.0
    }
}
