use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::time::Instant;

use crate::user::User;

/// Benchmark function that processes PyO3 users and returns timing info.
///
/// # Arguments
///
/// * `py` - Python GIL token
/// * `users` - A list of PyO3 User objects
///
/// # Returns
///
/// A dictionary with timing statistics and result summary
#[pyfunction]
pub fn benchmark_pyo3_process<'py>(py: Python<'py>, users: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyDict>> {
    let start = Instant::now();

    let mut total_age: i64 = 0;
    let mut active_count: i64 = 0;

    for i in 0..users.len()? {
        let user_obj = users.get_item(i)?;
        let user = user_obj.extract::<PyRef<User>>()?;

        // Direct field access - no dictionary lookup, fixed memory offset
        if user.active {
            total_age += user.age as i64;
            active_count += 1;
        }
    }

    let elapsed_us = start.elapsed().as_micros() as f64;

    let dict = PyDict::new(py);
    dict.set_item("total_age", total_age)?;
    dict.set_item("active_count", active_count)?;
    dict.set_item("elapsed_us", elapsed_us)?;
    Ok(dict)
}
