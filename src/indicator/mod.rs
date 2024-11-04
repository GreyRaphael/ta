use pyo3::prelude::*;
mod momentum;
mod overlap;

pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let indicator = PyModule::new_bound(parent_module.py(), "indicator")?;
    indicator.add_class::<momentum::ADX>()?;
    indicator.add_class::<momentum::ADXR>()?;
    indicator.add_class::<momentum::ULTOSC>()?;
    parent_module.add_submodule(&indicator)
}
