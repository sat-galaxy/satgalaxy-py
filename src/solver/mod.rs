mod minisat;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;
#[gen_stub_pyclass_enum(module = "satgalaxy.solver")]
#[pyclass(name="Status", module = "satgalaxy.solver")]
pub enum PyRawStatus {
    Satisfiable,
    Unsatisfiable,
    Unknown,
}

impl From<satgalaxy_rs::solver::RawStatus> for PyRawStatus {
    fn from(status: satgalaxy_rs::solver::RawStatus) -> Self {
        match status {
            satgalaxy_rs::solver::RawStatus::Satisfiable => Self::Satisfiable,
            satgalaxy_rs::solver::RawStatus::Unsatisfiable => Self::Unsatisfiable,
            satgalaxy_rs::solver::RawStatus::Unknown => Self::Unknown,
        }
    }
}

pub fn add_solver_module(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let solver_module = PyModule::new(py, "solver")?;
    minisat::add_minisat_module(py, &solver_module)?;
    solver_module.add_class::<PyRawStatus>()?;
    m.add_submodule(&solver_module)
}
