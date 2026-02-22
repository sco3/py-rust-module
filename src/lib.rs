use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Compute the sum of two integers.
///
/// # Examples
///
/// ```
/// let s = add(2, 3);
/// assert_eq!(s, 5);
/// ```
#[pyfunction]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two integers.
///
/// # Examples
///
/// ```
/// assert_eq!(multiply(2, 3), 6);
/// ```
#[pyfunction]
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

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
    /// Creates a new Calculator with its internal value set to `initial_value`.
    ///
    /// # Examples
    ///
    /// ```
    /// let calc = Calculator::new(3.5);
    /// assert_eq!(calc.value, 3.5);
    /// ```
    #[new]
    fn new(initial_value: f64) -> Self {
        Calculator { value: initial_value }
    }

    /// Adds `x` to the calculator's internal value and returns the updated total.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut c = Calculator::new(1.5);
    /// let v = c.add(2.0);
    /// assert_eq!(v, 3.5);
    /// ```
    fn add(&mut self, x: f64) -> f64 {
        self.value += x;
        self.value
    }

    /// Multiplies the calculator's internal value by the given factor.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut calc = Calculator::new(2.0);
    /// let result = calc.multiply(3.0);
    /// assert_eq!(result, 6.0);
    /// ```
    ///
    /// @returns `f64` — the updated internal value after multiplication.
    fn multiply(&mut self, x: f64) -> f64 {
        self.value *= x;
        self.value
    }

    /// Resets the calculator's internal value to 0.0 and returns the new value.
    ///
    /// # Returns
    ///
    /// The updated internal value (0.0).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut calc = Calculator::new(3.5);
    /// assert_eq!(calc.reset(), 0.0);
    /// ```
    fn reset(&mut self) -> f64 {
        self.value = 0.0;
        self.value
    }

    /// Formats a string representation of the calculator including its current value.
    ///
    /// # Returns
    ///
    /// A `String` in the form "Calculator(value={})" where `{}` is the calculator's current `value`.
    ///
    /// # Examples
    ///
    /// ```
    /// let calc = Calculator::new(3.14);
    /// assert_eq!(calc.__repr__(), "Calculator(value=3.14)");
    /// ```
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
    /// Creates a new User with the provided id, name, email, age, and active flag.
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string(), 30, true);
    /// assert_eq!(user.id, 1);
    /// assert_eq!(user.name, "Alice");
    /// assert_eq!(user.email, "alice@example.com");
    /// assert_eq!(user.age, 30);
    /// assert!(user.active);
    /// ```
    #[new]
    fn new(id: i32, name: String, email: String, age: i32, active: bool) -> Self {
        User { id, name, email, age, active }
    }

    /// Serialize the User to a compact JSON string.
    ///
    /// Returns `Ok(String)` containing the compact JSON representation of the user on success,
    /// or `Err(PyValueError)` containing the serialization error message on failure.
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User::new(1, "Alice".into(), "alice@example.com".into(), 30, true);
    /// let json = user.json().unwrap();
    /// assert!(json.contains("\"name\":\"Alice\""));
    /// ```
    fn json(&self) -> PyResult<String> {
        serde_json::to_string(self).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Serialize the user to a pretty-printed JSON string.
    ///
    /// Produces a human-readable, pretty-formatted JSON representation of the `User`.
    ///
    /// # Errors
    ///
    /// Returns a `PyValueError` if serialization fails.
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User::new(1, "Alice".into(), "alice@example.com".into(), 30, true);
    /// let s = user.json_pretty().unwrap();
    /// assert!(s.contains("\n")); // pretty output contains newlines
    /// ```
    fn json_pretty(&self) -> PyResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Creates a User instance from a JSON string.
    ///
    /// Parses `json_str` and returns the corresponding `User` value.
    ///
    /// # Errors
    ///
    /// Returns a `PyValueError` if `json_str` is not valid JSON or does not match the `User` schema.
    ///
    /// # Examples
    ///
    /// ```
    /// let json = r#"{"id":1,"name":"Alice","email":"alice@example.com","age":30,"active":true}"#.to_string();
    /// let user = User::from_json(json).unwrap();
    /// assert_eq!(user.id, 1);
    /// assert_eq!(user.name, "Alice");
    /// ```
    #[staticmethod]
    fn from_json(json_str: String) -> PyResult<Self> {
        serde_json::from_str(&json_str).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a Python dictionary containing the user's public fields.
    ///
    /// The returned dictionary has the keys "id", "name", "email", "age", and "active" mapped to the corresponding values from the User.
    ///
    /// # Examples
    ///
    /// ```
    /// use pyo3::prelude::*;
    /// use pyo3::types::PyDict;
    ///
    /// Python::with_gil(|py| {
    ///     let user = User::new(1, "Alice".into(), "alice@example.com".into(), 30, true);
    ///     let d: &PyDict = user.dict(py);
    ///     assert_eq!(d.get_item("id").unwrap().extract::<i32>().unwrap(), 1);
    ///     assert_eq!(d.get_item("name").unwrap().extract::<String>().unwrap(), "Alice");
    ///     assert_eq!(d.get_item("email").unwrap().extract::<String>().unwrap(), "alice@example.com");
    ///     assert_eq!(d.get_item("age").unwrap().extract::<i32>().unwrap(), 30);
    ///     assert_eq!(d.get_item("active").unwrap().extract::<bool>().unwrap(), true);
    /// });
    /// ```
    fn dict<'py>(&self, py: Python<'py>) -> Bound<'py, PyDict> {
        let dict = PyDict::new(py);
        dict.set_item("id", self.id).unwrap();
        dict.set_item("name", &self.name).unwrap();
        dict.set_item("email", &self.email).unwrap();
        dict.set_item("age", self.age).unwrap();
        dict.set_item("active", self.active).unwrap();
        dict
    }

    /// Return a new User with the same `id` and the provided updated fields.
    ///
    /// The returned `User` retains `self.id` while replacing `name`, `email`,
    /// `age`, and `active` with the supplied values.
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User::new(1, "Alice".into(), "alice@example.com".into(), 30, true);
    /// let updated = user.model_copy("Alice B".into(), "aliceb@example.com".into(), 31, false);
    /// assert_eq!(updated.id, user.id);
    /// assert_eq!(updated.name, "Alice B");
    /// assert_eq!(updated.email, "aliceb@example.com");
    /// assert_eq!(updated.age, 31);
    /// assert_eq!(updated.active, false);
    /// ```
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

    /// String representation of the user containing the id, name, and email.
    ///
    /// The returned string is formatted as `User(id={id}, name='{name}', email='{email}')`.
    ///
    /// # Examples
    ///
    /// ```
    /// let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string(), 30, true);
    /// assert_eq!(user.__repr__(), "User(id=1, name='Alice', email='alice@example.com')");
    /// ```
    fn __repr__(&self) -> String {
        format!("User(id={}, name='{}', email='{}')", self.id, self.name, self.email)
    }
}

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
fn process_pydantic_users(_py: Python<'_>, users: Bound<'_, PyAny>) -> PyResult<(i64, i64, f64)> {
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
fn process_pyo3_users(_py: Python<'_>, users: Bound<'_, PyAny>) -> PyResult<(i64, i64, f64)> {
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
fn benchmark_pydantic_process<'py>(py: Python<'py>, users: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyDict>> {
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
fn benchmark_pyo3_process<'py>(py: Python<'py>, users: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyDict>> {
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
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_class::<Calculator>()?;
    m.add_class::<User>()?;
    m.add_function(wrap_pyfunction!(process_pydantic_users, m)?)?;
    m.add_function(wrap_pyfunction!(process_pyo3_users, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_pydantic_process, m)?)?;
    m.add_function(wrap_pyfunction!(benchmark_pyo3_process, m)?)?;
    Ok(())
}