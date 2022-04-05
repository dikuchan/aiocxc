#![allow(unused)]

mod binance;
mod errors;
mod utils;

pub use errors::{Error, Result};

use pyo3::prelude::*;
use crate::binance::Binance;

// Using `name` because of the conflict with module names.
#[pyfunction]
#[pyo3(name = "binance")]
fn with_binance(key: String, secret: String) -> Binance {
    Binance::new(key, secret)
}

#[pymodule]
fn aiocxc(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(with_binance, m)?)?;

    Ok(())
}