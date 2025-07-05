pub mod solver;
use pyo3::prelude::*;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::gen_stub_pyfunction};

/// A Python module implemented in Rust.
#[pymodule]
fn satgalaxy(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    solver::add_solver_module(py, m)?;
    Ok(())
}
define_stub_info_gatherer!(stub_info);
