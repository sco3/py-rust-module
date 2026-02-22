use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde::{Deserialize, Serialize};

/// A User model with JSON serialization support (Pydantic-like)
#[pyclass(skip_from_py_object)]
#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    #[pyo3(get)]
    pub id: i32,
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub email: String,
    #[pyo3(get, set)]
    pub age: i32,
    #[pyo3(get, set)]
    pub active: bool,
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
