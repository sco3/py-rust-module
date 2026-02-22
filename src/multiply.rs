use pyo3::prelude::*;

/// Multiplies two integers.
///
/// # Examples
///
/// ```
/// assert_eq!(multiply(2, 3), 6);
/// ```
#[pyfunction]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
