use pyo3::prelude::*;
mod sum;
mod mean;
// You don't need to make them pub mod unless you want them accessible from outside the rolling module.

pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let rolling = PyModule::new_bound(parent_module.py(), "rolling")?;
    rolling.add_class::<sum::Sumer>()?;
    rolling.add_class::<mean::Meaner>()?;
    parent_module.add_submodule(&rolling)
}