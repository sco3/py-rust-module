use pyo3::prelude::*;
use std::time::Instant;

/// Process a list of Pydantic User objects using getattr for attribute access.
///
/// This function demonstrates the "Border Tax" - the overhead of accessing Python object
/// attributes through getattr (dictionary lookup, string hashing) vs direct field access.
///
/// # Arguments
///
/// * `py` - Python GIL token
/// * `users` - A list of Python objects (expected to be Pydantic User instances)
///
/// # Returns
///
/// A tuple containing:
/// - The sum of ages for active users
/// - The count of active users
/// - The elapsed time in microseconds
///
/// # Note
///
/// Uses PyResult to handle potential AttributeError during getattr, which is part of the "tax"
#[pyfunction]
pub fn process_pydantic_users(_py: Python<'_>, users: Bound<'_, PyAny>) -> PyResult<(i64, i64, f64)> {
    let start = Instant::now();

    let mut total_age: i64 = 0;
    let mut active_count: i64 = 0;

    // Iterate through the Python list
    for user_obj in users.try_iter()? {
        let user_obj = user_obj?;


        // getattr involves string hash + dictionary lookup - this is the "Access Tax"
        let active: bool = user_obj.getattr("active")?.extract()?;

        if active {
            let age: i32 = user_obj.getattr("age")?.extract()?;
            total_age += age as i64;
            active_count += 1;
        }
    }

    let elapsed = start.elapsed().as_micros() as f64;
    Ok((total_age, active_count, elapsed))
}
