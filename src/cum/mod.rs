use pyo3::prelude::*;
mod statis;

pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let cum = PyModule::new_bound(parent_module.py(), "cum")?;
    cum.add_class::<statis::Counter>()?;
    cum.add_class::<statis::Sumer>()?;
    cum.add_class::<statis::Meaner>()?;
    parent_module.add_submodule(&cum)
}