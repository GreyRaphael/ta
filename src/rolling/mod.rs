use pyo3::prelude::*;
mod sum;
mod minmax;
mod delta;
// You don't need to make them pub mod unless you want them accessible from outside the rolling module.

pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let rolling = PyModule::new_bound(parent_module.py(), "rolling")?;
    rolling.add_class::<sum::Sumer>()?;
    rolling.add_class::<sum::Meaner>()?;
    rolling.add_class::<minmax::Maxer>()?;
    rolling.add_class::<minmax::Miner>()?;
    rolling.add_class::<delta::Deltaer>()?;
    rolling.add_class::<delta::Pctchanger>()?;
    parent_module.add_submodule(&rolling)
}