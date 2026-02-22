use pyo3::prelude::*;
use std::time::Instant;

use crate::user::User;

/// Process a list of PyO3 User objects using direct field access.
///
/// This function demonstrates the performance advantage of direct field access
/// through PyO3's #[pyclass] - values are at fixed memory offsets.
///
/// # Arguments
///
/// * `py` - Python GIL token
/// * `users` - A list of PyO3 User objects
///
/// # Returns
///
/// A tuple containing:
/// - The sum of ages for active users
/// - The count of active users
/// - The elapsed time in microseconds
#[pyfunction]
pub fn process_pyo3_users(_py: Python<'_>, users: Bound<'_, PyAny>) -> PyResult<(i64, i64, f64)> {
    let start = Instant::now();

    let mut total_age: i64 = 0;
    let mut active_count: i64 = 0;

    // Iterate through the Python list and extract PyO3 User references
    for user_obj in users.try_iter()? {
        let user_obj = user_obj?;

        // Extract the PyO3 User - this is the "Entry Tax" (one-time conversion)
        let user = user_obj.extract::<PyRef<User>>()?;

        // Direct field access - no dictionary lookup, fixed memory offset
        if user.active {
            total_age += user.age as i64;
            active_count += 1;
        }
    }

    let elapsed = start.elapsed().as_micros() as f64;
    Ok((total_age, active_count, elapsed))
}
