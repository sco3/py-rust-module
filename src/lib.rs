mod add;
mod benchmark_pydantic_process;
mod benchmark_pyo3_process;
mod calculator;
mod greet;
mod multiply;
mod process_pydantic_users;
mod process_pyo3_users;
mod user;

use pyo3::prelude::*;

/// Initializes the Python module and registers the free functions and classes exposed to Python.
///
/// This function is the PyO3 module initializer; it adds the `add`, `multiply`, and `greet` functions
/// and registers the `Calculator` and `User` classes on the provided Python module.
///
/// # Examples
///
/// ```
/// use pyo3::prelude::*;
/// use pyo3::types::PyModule;
///
/// Python::with_gil(|py| {
///     let m = PyModule::new(py, "py_rust_module").unwrap();
///     // Module initialization (normally done automatically on import)
///     py_rust_module(m).unwrap();
///     // After initialization, `m` contains `add`, `multiply`, `greet`, `Calculator`, and `User`.
/// });
/// ```
#[pymodule]
fn py_rust_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add::add, m)?)?;
    m.add_function(wrap_pyfunction!(multiply::multiply, m)?)?;
    m.add_function(wrap_pyfunction!(greet::greet, m)?)?;
    m.add_class::<calculator::Calculator>()?;
    m.add_class::<user::User>()?;
    m.add_function(wrap_pyfunction!(process_pydantic_users::process_pydantic_users, m)?)?;
    m.add_function(wrap_pyfunction!(process_pyo3_users::process_pyo3_users, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_pydantic_process::benchmark_pydantic_process, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_pyo3_process::benchmark_pyo3_process, m)?)?;
    Ok(())
}
