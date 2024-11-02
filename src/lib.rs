use pyo3::prelude::*;
mod cum;
mod rolling;
mod indicator;
pub mod utils;

/// A Python module implemented in Rust.
#[pymodule]
fn ta(m: &Bound<'_, PyModule>) -> PyResult<()> {
    rolling::register(m)?;
    cum::register(m)?;
    indicator::register(m)?;
    Ok(())
}
