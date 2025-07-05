use pyo3::prelude::*;
use pyo3_stub_gen::{
    define_stub_info_gatherer,
    derive::{gen_stub_pyclass, gen_stub_pyclass_enum, gen_stub_pyfunction, gen_stub_pymethods},
};

use crate::solver::PyRawStatus;

#[gen_stub_pyclass(module = "satgalaxy.solver.minisat")]
#[pyclass(name = "MinisatSolver", module = "satgalaxy.solver.minisat")]
/// The minisat solver
pub struct Solver(satgalaxy_rs::solver::MinisatSolver);

#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// The variable activity decay factor(0~1). default: 0.95
pub fn set_opt_var_decay(decay: f64) -> PyResult<()> {
    satgalaxy_rs::solver::MinisatSolver::set_opt_var_decay(decay)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// The clause activity decay factor default: 0.999
pub fn set_opt_clause_decay(decay: f64) -> PyResult<()> {
    satgalaxy_rs::solver::MinisatSolver::set_opt_clause_decay(decay)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// The frequency with which the decision heuristic tries to choose a random variable
pub fn set_opt_random_var_freq(freq: f64)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_random_var_freq(freq).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// The frequency with which the decision heuristic tries to choose a random variable
pub fn set_opt_random_seed(seed: f64)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_random_seed(seed).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Controls conflict clause minimization (0=none, 1=basic, 2=deep). return error code, 0 for success, others for invalid parameter
pub fn set_opt_ccmin_mode(mode: i32)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_ccmin_mode(mode).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Controls the level of phase saving (0=none, 1=limited, 2=full). return error code, 0 for success, others for invalid parameter
pub fn set_opt_phase_saving(mode: i32)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_phase_saving(mode).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Randomize the initial activity
pub fn set_opt_rnd_init_act(flag: bool)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_rnd_init_act(flag.into()).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Use the Luby restart sequence
pub fn set_opt_luby_restart(flag: bool)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_luby_restart(flag.into()).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// The base restart interval, 100 for default
pub fn set_opt_restart_first(restart_first: i32)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_restart_first(restart_first).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Restart interval increase factor, must be at least 1.0
pub fn set_opt_restart_inc(restart_inc: f64)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_restart_inc(restart_inc).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Restart interval increase factor, must be at least 0
pub fn set_opt_min_learnts_lim(min_learnts_lim: i32)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_min_learnts_lim(min_learnts_lim).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Shrink clauses by asymmetric branching
pub fn set_opt_use_asymm(opt_use_asymm: bool)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_use_asymm(opt_use_asymm.into()).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Check if a clause is already implied
pub fn set_opt_use_rcheck(opt_use_rcheck: bool)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_use_rcheck(opt_use_rcheck.into()).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Perform variable elimination
pub fn set_opt_use_elim(opt_use_elim: bool)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_use_elim(opt_use_elim.into()).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Allow a variable elimination step to grow by a number of clauses, must be at least 0
pub fn set_opt_grow(opt_grow: i32)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_grow(opt_grow).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Variables are not eliminated if it produces a resolvent with a length above this limit. -1 means no limit. Must be at least -1
pub fn set_opt_clause_lim(opt_clause_lim: i32)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_clause_lim(opt_clause_lim).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// Do not check if subsumption against a clause larger than this. -1 means no limit. Must be at least -1
pub fn set_opt_subsumption_lim(opt_subsumption_lim: i32)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_subsumption_lim(opt_subsumption_lim).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// The fraction of wasted memory allowed before a garbage collection is triggered during simplification, must be positive.
pub fn set_opt_simp_garbage_frac(opt_simp_garbage_frac: f64)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_simp_garbage_frac(opt_simp_garbage_frac).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// The fraction of wasted memory allowed before a garbage collection is triggered, must be positive.
pub fn set_opt_garbage_frac(garbage_frac: f64)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_garbage_frac(garbage_frac).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}
#[gen_stub_pyfunction(module = "satgalaxy.solver.minisat")]
#[pyfunction]
/// The verbosity level, 0=silent, 1=some, 2=verbose
//#[pyo3(signature=(verb=5))]
pub fn set_opt_verbosity(verb: i32)-> PyResult<()>{
    satgalaxy_rs::solver::MinisatSolver::set_opt_verbosity(verb).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}



#[gen_stub_pymethods]
#[pymethods]
impl Solver {
    #[new]
    /// create a new solver
    pub fn new(ds:i32) -> Self {
        Self(satgalaxy_rs::solver::MinisatSolver::new())
    }
    /// The current number of variables.
    pub fn vars(& self) -> i32 {
        self.0.vars()
    }
    /// Create a new variable
    pub fn new_var(& self) -> i32 {
        self.0.new_var()
    }
    /// Release a variable.
    pub fn release_var(& self, var: i32) {
        self.0.release_var(var)

    }
    /// Add a clause to the solver.
    pub fn add_clause(& self, clause: Vec<i32>) {
        self.0.add_clause(&clause);
    }
    /// Add an empty clause to the solver. (unsat)
    pub fn add_empty_clause(& self) {
        self.0.add_empty_clause();
    }
    ///  The current assignments for the variables
    pub fn value(& self, var: i32) -> bool {
        self.0.value(var)
    }
    /// The model assignments for the variables
    pub fn model_value(& self, var: i32) -> bool {
        self.0.model_value(var)
    }
    /// Solving with assumptions, do_simp (recommend true) and turn_off_simp (recommend false)
    #[pyo3(signature = (assumps,do_simp=true,turn_off_simp=false))]
    pub fn solve_assumps(& self, assumps: Vec<i32>,  do_simp: bool, turn_off_simp: bool) -> bool {
        self.0.solve_assumps(&assumps, do_simp, turn_off_simp)
    }
    /// Solving, do_simp (recommend true) and turn_off_simp (recommend false)
    #[pyo3(signature = (assumps,do_simp=true,turn_off_simp=false))]
    pub fn solve_limited(
        & self,
        assumps: Vec<i32>,
        do_simp: bool,
        turn_off_simp: bool,
    ) -> PyRawStatus {
        self.0.solve_limited(&assumps, do_simp, turn_off_simp).into()


    }
    /// Solving, do_simp (recommend true) and turn_off_simp (recommend false)
    // #[pyo3(signature = (do_simp=true,turn_off_simp=false))]
    pub fn solve(& self, do_simp: bool, turn_off_simp: bool) -> bool {
        self.0.solve(do_simp, turn_off_simp)
    }
    /// Perform variable elimination based simplification. turn_off_simp (recommend false)
    // #[pyo3(signature = (turn_off_simp=false))]
    pub fn eliminate(& self, turn_off_simp: bool) {
        self.0.eliminate(turn_off_simp);
    }
    /// The current number of assigned literals.
    pub fn assigns(& self) -> usize {
        self.0.assigns()
    }
    /// The current number of original clauses.
    pub fn clauses(& self) -> usize {
        self.0.clauses()
    }
    /// The current number of learnt clauses.
    pub fn learnts(& self) -> usize {
        self.0.learnts()
    }

    pub fn okay(& self) -> bool {
        self.0.okay()
    }
    /// Get current model if the solver is satisfiable.
    pub fn model(& self) -> Vec<i32> {
        self.0.model()
    }
}

pub fn add_minisat_module(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let solver_module = PyModule::new(py, "minisat")?;
    solver_module.add_function(wrap_pyfunction!(set_opt_var_decay, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_clause_decay, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_random_var_freq, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_random_seed, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_ccmin_mode, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_phase_saving, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_rnd_init_act, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_luby_restart, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_restart_first, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_restart_inc, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_min_learnts_lim, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_use_asymm, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_use_rcheck, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_use_elim, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_grow, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_clause_lim, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_subsumption_lim, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_simp_garbage_frac, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_garbage_frac, &solver_module)?)?;
    solver_module.add_function(wrap_pyfunction!(set_opt_verbosity, &solver_module)?)?;
    solver_module.add_class::<Solver>()?;
    solver_module.add_class::<PyRawStatus>()?;
    m.add_submodule(&solver_module)
}
