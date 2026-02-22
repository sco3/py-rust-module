use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::time::Instant;

/// Benchmark function that processes Pydantic users and returns timing info.
///
/// # Arguments
///
/// * `py` - Python GIL token
/// * `users` - A list of Python objects (Pydantic User instances)
///
/// # Returns
///
/// A dictionary with timing statistics and result summary
#[pyfunction]
pub fn benchmark_pydantic_process<'py>(py: Python<'py>, users: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyDict>> {
    let start = Instant::now();

    let mut total_age: i64 = 0;
    let mut active_count: i64 = 0;
    let mut errors: i64 = 0;

    for i in 0..users.len()? {
        let user_obj = users.get_item(i)?;
        match user_obj.getattr("active") {
            Ok(active_val) => {
                if let Ok(active) = active_val.extract::<bool>() {
                    if active {
                        if let Ok(age_val) = user_obj.getattr("age") {
                            if let Ok(age) = age_val.extract::<i32>() {
                                total_age += age as i64;
                                active_count += 1;
                            }
                        }
                    }
                }
            }
            Err(_) => errors += 1,
        }
    }

    let elapsed_us = start.elapsed().as_micros() as f64;

    let dict = PyDict::new(py);
    dict.set_item("total_age", total_age)?;
    dict.set_item("active_count", active_count)?;
    dict.set_item("errors", errors)?;
    dict.set_item("elapsed_us", elapsed_us)?;
    Ok(dict)
}
