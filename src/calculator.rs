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
    /// ```ignore
    /// calc = Calculator(3.5)
    /// calc.value
    /// ```
    #[new]
    fn new(initial_value: f64) -> Self {
        Calculator { value: initial_value }
    }

    /// Adds `x` to the calculator's internal value and returns the updated total.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// c = Calculator(1.5)
    /// c.add(2.0)
    /// ```
    fn add(&mut self, x: f64) -> f64 {
        self.value += x;
        self.value
    }

    /// Multiplies the calculator's internal value by the given factor.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// calc = Calculator(2.0)
    /// calc.multiply(3.0)
    /// ```
    /// # Returns
    /// `f64` — the updated internal value after multiplication.
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
    /// ```ignore
    /// calc = Calculator(3.5)
    /// calc.reset()
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
    /// ```ignore
    /// calc = Calculator(3.14)
    /// repr(calc)
    /// ```
    fn __repr__(&self) -> String {
        format!("Calculator(value={})", self.value)
    }
}
