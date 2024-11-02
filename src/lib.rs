use pyo3::prelude::*;
mod rolling;

/// A Python module implemented in Rust.
#[pymodule]
fn ta(m: &Bound<'_, PyModule>) -> PyResult<()> {
    rolling::register(m)?;
    Ok(())
}
