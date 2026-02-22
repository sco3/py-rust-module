use pyo3::prelude::*;

/// Compute the sum of two integers.
///
/// # Examples
///
/// ```
/// let s = add(2, 3);
/// assert_eq!(s, 5);
/// ```
#[pyfunction]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
