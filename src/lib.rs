use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

/// Add two numbers
#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiply two numbers
#[pyfunction]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Greet someone
#[pyfunction]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// A simple calculator with stateful operations
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
struct Calculator {
    #[pyo3(get, set)]
    value: f64,
}

#[pymethods]
impl Calculator {
    #[new]
    fn new(initial_value: f64) -> Self {
        Calculator { value: initial_value }
    }

    fn add(&mut self, x: f64) -> f64 {
        self.value += x;
        self.value
    }

    fn multiply(&mut self, x: f64) -> f64 {
        self.value *= x;
        self.value
    }

    fn reset(&mut self) -> f64 {
        self.value = 0.0;
        self.value
    }

    fn __repr__(&self) -> String {
        format!("Calculator(value={})", self.value)
    }
}

/// A User model with JSON serialization support (Pydantic-like)
#[pyclass(skip_from_py_object)]
#[derive(Clone, Serialize, Deserialize)]
struct User {
    #[pyo3(get)]
    id: i32,
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    email: String,
    #[pyo3(get, set)]
    age: i32,
    #[pyo3(get, set)]
    active: bool,
}

#[pymethods]
impl User {
    #[new]
    fn new(id: i32, name: String, email: String, age: i32, active: bool) -> Self {
        User { id, name, email, age, active }
    }

    /// Serialize to JSON string
    fn json(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Serialize to pretty JSON string
    fn json_pretty(&self) -> PyResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create User from JSON string
    #[staticmethod]
    fn from_json(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Convert to dictionary
    fn dict<'py>(&self, py: Python<'py>) -> Bound<'py, PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("id", self.id).unwrap();
        dict.set_item("name", &self.name).unwrap();
        dict.set_item("email", &self.email).unwrap();
        dict.set_item("age", self.age).unwrap();
        dict.set_item("active", self.active).unwrap();
        dict
    }

    /// Create a copy with modified fields
    #[pyo3(signature = (name, email, age, active))]
    fn model_copy(&self, name: String, email: String, age: i32, active: bool) -> Self {
        User {
            id: self.id,
            name,
            email,
            age,
            active,
        }
    }

    fn __repr__(&self) -> String {
        format!("User(id={}, name='{}', email='{}')", self.id, self.name, self.email)
    }
}

/// A Python module written in Rust using PyO3
#[pymodule]
fn py_rust_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_class::<Calculator>()?;
    m.add_class::<User>()?;
    Ok(())
}
