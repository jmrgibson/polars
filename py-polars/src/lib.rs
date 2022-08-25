//! Exposes python polars types as cdylib module.
//! 

use pyo3::prelude::*;

#[pymodule]
fn polars(py: Python, m: &PyModule) -> PyResult<()> {
    py_polars_core::add_types_to_module(py, m)
}