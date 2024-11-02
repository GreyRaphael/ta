use pyo3::prelude::*;
pub mod sum;
pub mod mean;

pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let rolling = PyModule::new_bound(parent_module.py(), "rolling")?;
    rolling.add_class::<sum::Sumer>()?;
    rolling.add_class::<mean::Meaner>()?;
    parent_module.add_submodule(&rolling)
}