use pyo3::prelude::*;

/// Generates a greeting for the given name.
///
/// The returned string is formatted as "Hello, {name}!".
///
/// # Examples
///
/// ```
/// let s = greet("Alice");
/// assert_eq!(s, "Hello, Alice!");
/// ```
#[pyfunction]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
