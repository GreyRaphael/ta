use pyo3::prelude::*;
mod minmax;
pub mod delta;
pub mod statis;
mod ema;
mod quantile;
mod corr;
// You don't need to make them pub mod unless you want them accessible from outside the rolling module.

pub fn register(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let rolling = PyModule::new_bound(parent_module.py(), "rolling")?;
    rolling.add_class::<statis::Sumer>()?;
    rolling.add_class::<statis::Meaner>()?;
    rolling.add_class::<minmax::Maxer>()?;
    rolling.add_class::<minmax::Miner>()?;
    rolling.add_class::<delta::Deltaer>()?;
    rolling.add_class::<delta::Pctchanger>()?;
    rolling.add_class::<statis::Stder>()?;
    rolling.add_class::<statis::Skewer>()?;
    rolling.add_class::<statis::Kurter>()?;
    rolling.add_class::<ema::EMAer>()?;
    rolling.add_class::<corr::Correlationer>()?;
    rolling.add_class::<quantile::Quantiler>()?;
    parent_module.add_submodule(&rolling)
}