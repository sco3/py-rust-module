use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::time::Instant;

/// Extracts specific data from a Python object, returning None if
/// the object is inactive or malformed.
fn try_get_active_age(obj: &Bound<'_, PyAny>) -> PyResult<Option<i32>> {
    let active: bool = obj.getattr("active")?.extract()?;
    if !active {
        return Ok(None);
    }

    let age: i32 = obj.getattr("age")?.extract()?;
    Ok(Some(age))
}

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
pub fn benchmark_pydantic_process<'py>(
    py: Python<'py>,
    users: Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyDict>> {
    let start = Instant::now();

    let mut total_age: i64 = 0;
    let mut active_count: i64 = 0;
    let mut errors: i64 = 0;

    for user_obj_res in users.try_iter()? {
        let user_obj = user_obj_res?;

        match try_get_active_age(&user_obj) {
            Ok(Some(age)) => {
                total_age += age as i64;
                active_count += 1;
            }
            Ok(None) => {}
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
