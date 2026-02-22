use pyo3::prelude::*;

/// A simple calculator with stateful operations
#[pyclass(skip_from_py_object)]
#[derive(Clone)]
pub struct Calculator {
    #[pyo3(get, set)]
    pub value: f64,
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
