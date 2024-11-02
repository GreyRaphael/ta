use pyo3::prelude::*;
pub mod sum;

pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let rolling = PyModule::new_bound(parent_module.py(), "rolling")?;
    rolling.add_class::<sum::Sumer>()?;
    parent_module.add_submodule(&rolling)
}