use pyo3::prelude::*;
mod corr;
mod delta;
mod minmax;
mod quantile;
mod statis;

pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let cum = PyModule::new_bound(parent_module.py(), "cum")?;
    cum.add_class::<statis::Counter>()?;
    cum.add_class::<statis::Sumer>()?;
    cum.add_class::<statis::Meaner>()?;
    cum.add_class::<statis::Stder>()?;
    cum.add_class::<statis::Skewer>()?;
    cum.add_class::<statis::Kurter>()?;
    cum.add_class::<minmax::Maxer>()?;
    cum.add_class::<minmax::Miner>()?;
    cum.add_class::<corr::Correlationer>()?;
    cum.add_class::<delta::Deltaer>()?;
    cum.add_class::<delta::Pctchanger>()?;
    cum.add_class::<quantile::Quantiler>()?;
    parent_module.add_submodule(&cum)
}
