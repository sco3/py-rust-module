# PyO3 Project - py_rust_module

A high-performance Python module written in Rust using PyO3. This project demonstrates how to create fast Python extensions using Rust, with a Pydantic-like User model that outperforms pure Python implementations.

## Features

- ✨ Simple functions (`add`, `multiply`, `greet`)
- 🧮 Calculator class with stateful operations
- 📦 Pydantic-like User model with JSON serialization
- 🚀 **5-17x faster** than Pydantic for JSON operations
- 🐍 Easy to use from Python
- 🦀 Built with Rust + PyO3 0.28

## Performance Benchmarks

### JSON Operations: Rust `py_rust_module.User` vs Pydantic `User`

| Operation | Rust (μs) | Pydantic (μs) | Speedup |
|-----------|-----------|---------------|---------|
| `json()` | 0.15 | 0.80 | **5.28x** |
| `json_pretty()` | 0.16 | 0.95 | **5.92x** |
| `from_json()` | 0.20 | 0.87 | **4.27x** |
| `dict()` | 0.26 | 0.73 | **2.76x** |
| `model_copy()` | 0.13 | 1.12 | **8.83x** |

*Benchmark: 100,000 iterations on Python 3.14*

---

### Border Tax Benchmark: Pydantic V2 vs PyO3 `#[pyclass]`

This benchmark compares attribute access patterns between Pydantic models and PyO3-backed classes.
The test processes **100,000 User instances**, performing conditional age summation (`if active { total += age }`).

| Benchmark | Pydantic (μs) | PyO3 (μs) | Speedup |
|-----------|---------------|-----------|---------|
| `process_pydantic_users` (getattr + PyResult) | 9,850 | 2,125 | **4.64x** |
| `process_pyo3_users` (direct field access) | 9,861 | 2,304 | **4.28x** |

*Benchmark: 100,000 users, 10 iterations, Python 3.14. Both implementations operate on identical shared data with verified matching results.*

#### The "Border Tax" Explained

The **~7,700 μs delta** represents the performance penalty when accessing Python object attributes from Rust:

| Tax Type | Pydantic Overhead | PyO3 Advantage |
|----------|-------------------|----------------|
| **Entry Tax** | `Vec<PyObject>` - raw Python objects | `Vec<PyRef<User>>` - direct Rust struct access |
| **Access Tax** | `getattr()` → string hash + dict lookup on Python heap | Direct field access at fixed memory offset |
| **Error Tax** | `PyResult` handles potential `AttributeError` | Fields guaranteed to exist - no error handling |

**Why the conditional summation?**

The logic `if active { total_age += age }` prevents compiler optimization by:
- Actually using retrieved values (not a no-op)
- Creating a data dependency that forces real memory access
- Requiring both the `active` check AND `age` retrieval

This benchmark demonstrates why PyO3 `#[pyclass]` is beneficial for performance-critical code that processes many objects.

## Prerequisites

- [uv](https://github.com/astral-sh/uv) - Python package manager
- Rust (with Cargo) - [Install from rustup.rs](https://rustup.rs/)

## Quick Start with uv

```bash
# Clone and enter the project
cd py-rust-module

# Build and install the module
uv pip install -e .

# Run the example
uv run python example.py

# Run benchmarks
uv run python benchmark.py
```

## Manual Build with Maturin

```bash
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop

# Or for a release build (optimized)
maturin develop --release
```

## Usage

### Basic Functions

```python
import py_rust_module

# Use functions
result = py_rust_module.add(5, 3)
print(f"5 + 3 = {result}")

result = py_rust_module.multiply(4, 7)
print(f"4 × 7 = {result}")

greeting = py_rust_module.greet("World")
print(greeting)  # Hello, World!
```

### Calculator Class

```python
calc = py_rust_module.Calculator(10.0)
print(f"Initial value: {calc.value}")

calc.add(5.0)
print(f"After adding 5: {calc.value}")  # 15.0

calc.multiply(2.0)
print(f"After multiplying by 2: {calc.value}")  # 30.0

calc.reset()
print(f"After reset: {calc.value}")  # 0.0
```

### User Model with JSON Support

```python
import py_rust_module

# Create a user
user = py_rust_module.User(
    id=1,
    name="Alice Johnson",
    email="alice@example.com",
    age=30,
    active=True
)

# Serialize to JSON
json_str = user.json()
print(json_str)
# {"id":1,"name":"Alice Johnson","email":"alice@example.com","age":30,"active":true}

# Pretty print JSON
print(user.json_pretty())

# Convert to dictionary
user_dict = user.dict()

# Deserialize from JSON
user2 = py_rust_module.User.from_json(json_str)

# Create a modified copy
user3 = user.model_copy(
    name="Alice Smith",
    email=user.email,
    age=31,
    active=user.active
)
```

**User Model Methods:**
- `.json()` - Serialize to compact JSON string
- `.json_pretty()` - Serialize to pretty-printed JSON string
- `.from_json(json_str)` - Static method to create User from JSON
- `.dict()` - Convert to Python dictionary
- `.model_copy(...)` - Create a modified copy with updated fields

## Running Examples and Benchmarks

```bash
# Run example usage
uv run python example.py

# Run performance benchmarks
uv run python benchmark.py

# Compare with Pydantic
uv run python pydantic_model.py
```

## Building a Wheel

```bash
# Using uv
uv build

# Using maturin
maturin build --release
```

The wheel will be created in `dist/`.

## Installing from Wheel

```bash
pip install dist/py_rust_module-*.whl
```

## Project Structure

```
py-rust-module/
├── Cargo.toml          # Rust package configuration
├── pyproject.toml      # Python package configuration (uv/maturin)
├── README.md           # This file
├── example.py          # Example usage
├── benchmark.py        # Performance benchmarks
├── pydantic_model.py   # Pydantic equivalent for comparison
└── src/
    └── lib.rs          # Rust source code with PyO3 bindings
```

## Development

### Building

```bash
# Clean build
rm -rf target dist
uv build

# Development mode
maturin develop
```

### Adding New Functions

1. Add your function in `src/lib.rs` with the `#[pyfunction]` attribute
2. Register it in the `#[pymodule]` function
3. Rebuild with `maturin develop` or `uv pip install -e .`

### Adding New Classes

1. Define a struct with `#[pyclass]`
2. Implement methods with `#[pymethods]`
3. Register the class in the `#[pymodule]` function
4. Rebuild

### Adding JSON Serialization

1. Add `#[derive(Serialize, Deserialize)]` to your struct
2. Implement `.json()`, `.json_pretty()`, and `.from_json()` methods
3. Use `serde_json` for serialization/deserialization

## Dependencies

**Rust (Cargo.toml):**
- `pyo3` - Python bindings for Rust
- `serde` + `serde_json` - JSON serialization

**Python (pyproject.toml):**
- `maturin` - Build tool for Rust extensions
- `pydantic` - For comparison benchmarks

## Resources

- [PyO3 Documentation](https://pyo3.rs/)
- [Maturin Documentation](https://www.maturin.rs/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Serde JSON](https://docs.rs/serde_json/)
- [Pydantic Documentation](https://docs.pydantic.dev/)

## License

This project is open source and available under the MIT License.
