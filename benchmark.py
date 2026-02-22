#!/usr/bin/env python3
"""
High-performance benchmark comparing Pydantic V2 model vs PyO3 #[pyclass]
for a User model with conditional age summation.

This benchmark demonstrates the "Border Tax" - the performance penalty
when crossing the Python/Rust boundary and accessing attributes.
"""
import random
import statistics
import time
from typing import List, Tuple

from pydantic_model import User as PydanticUser

try:
    from py_rust_module import py_rust_module
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False
    print("Warning: py_rust_module not available, skipping Rust benchmarks")


# Number of users in the test dataset
NUM_USERS = 100_000

# Number of benchmark iterations
BENCHMARK_ITERATIONS = 10


def generate_pydantic_users(count: int) -> List[PydanticUser]:
    """Generate a list of Pydantic User instances with random data."""
    users = []
    for i in range(count):
        users.append(PydanticUser(
            id=i,
            name=f"User_{i}",
            email=f"user{i}@example.com",
            age=random.randint(18, 80),
            active=random.choice([True, False]),
        ))
    return users


def generate_pyo3_users(count: int) -> List["py_rust_module.User"]:
    """Generate a list of PyO3 User instances with random data."""
    users = []
    for i in range(count):
        users.append(py_rust_module.User(
            id=i,
            name=f"User_{i}",
            email=f"user{i}@example.com",
            age=random.randint(18, 80),
            active=random.choice([True, False]),
        ))
    return users


def benchmark_function(func, *args, iterations: int = BENCHMARK_ITERATIONS, name: str = "") -> dict:
    """
    Benchmark a function and return timing statistics.
    
    Args:
        func: The function to benchmark
        *args: Arguments to pass to the function
        iterations: Number of iterations to run
        name: Name for the benchmark
        
    Returns:
        Dictionary with timing statistics
    """
    times = []
    results = []
    
    for _ in range(iterations):
        start = time.perf_counter()
        result = func(*args)
        end = time.perf_counter()
        times.append((end - start) * 1_000_000)  # Convert to microseconds
        results.append(result)
    
    # Verify results are consistent (same total_age and active_count)
    if results:
        first_result = results[0]
        for r in results[1:]:
            if isinstance(first_result, tuple) and isinstance(r, tuple):
                assert first_result[0] == r[0], f"Inconsistent total_age: {first_result[0]} vs {r[0]}"
                assert first_result[1] == r[1], f"Inconsistent active_count: {first_result[1]} vs {r[1]}"
    
    return {
        "name": name,
        "mean": statistics.mean(times),
        "median": statistics.median(times),
        "stdev": statistics.stdev(times) if len(times) > 1 else 0,
        "min": min(times),
        "max": max(times),
        "result": results[0] if results else None,
    }


def print_results_table(results: List[dict]) -> None:
    """Print benchmark results as a formatted table."""
    print(f"\n{'Benchmark':<35} {'Mean (μs)':<14} {'Median (μs)':<14} {'Stdev':<10} {'Min (μs)':<12} {'Max (μs)':<12}")
    print("-" * 107)
    for r in results:
        print(f"{r['name']:<35} {r['mean']:<14.2f} {r['median']:<14.2f} {r['stdev']:<10.2f} {r['min']:<12.2f} {r['max']:<12.2f}")


def print_border_tax_analysis(pydantic_time: float, pyo3_time: float, label: str) -> None:
    """
    Print the "Border Tax" analysis comparing two execution times.
    
    The Border Tax represents the overhead of:
    1. Entry Tax: Converting Python objects to Rust types
    2. Access Tax: getattr (dict lookup + hash) vs direct field access
    """
    speedup = pydantic_time / pyo3_time if pyo3_time > 0 else float('inf')
    delta = pydantic_time - pyo3_time
    delta_pct = (delta / pydantic_time * 100) if pydantic_time > 0 else 0
    
    print(f"\n  {label}:")
    print(f"    Pydantic:  {pydantic_time:>10.2f} μs")
    print(f"    PyO3:      {pyo3_time:>10.2f} μs")
    print(f"    Delta:     {delta:>10.2f} μs ({delta_pct:.1f}%)")
    print(f"    Speedup:   {speedup:>10.2f}x faster with PyO3")
    print()
    print("    Border Tax Breakdown:")
    print(f"      - Entry Tax:  Python list -> Rust Vec conversion overhead")
    print(f"      - Access Tax: getattr (hash + dict lookup) vs direct field access")
    print(f"      - Error Tax:  PyResult handling for potential AttributeError")


def main():
    """Run the benchmark suite."""
    print("=" * 107)
    print("Benchmark: Pydantic V2 vs PyO3 #[pyclass] - User Model Performance")
    print(f"Dataset: {NUM_USERS:,} users | Iterations: {BENCHMARK_ITERATIONS}")
    print("=" * 107)
    
    if not RUST_AVAILABLE:
        print("\nERROR: py_rust_module not available. Build with: maturin develop")
        return
    
    # Generate test datasets
    print("\nGenerating test datasets...")
    start = time.perf_counter()
    pydantic_users = generate_pydantic_users(NUM_USERS)
    pydantic_gen_time = (time.perf_counter() - start) * 1000
    print(f"  Pydantic users generated: {pydantic_gen_time:.2f} ms")
    
    start = time.perf_counter()
    pyo3_users = generate_pyo3_users(NUM_USERS)
    pyo3_gen_time = (time.perf_counter() - start) * 1000
    print(f"  PyO3 users generated: {pyo3_gen_time:.2f} ms")
    
    # Verify datasets have similar distributions
    pydantic_active = sum(1 for u in pydantic_users if u.active)
    pyo3_active = sum(1 for u in pyo3_users if u.active)
    print(f"\n  Pydantic active users: {pydantic_active:,} ({pydantic_active/NUM_USERS*100:.1f}%)")
    print(f"  PyO3 active users:     {pyo3_active:,} ({pyo3_active/NUM_USERS*100:.1f}%)")
    
    results = []
    
    # =========================================================================
    # Benchmark 1: process_pydantic_users vs process_pyo3_users
    # =========================================================================
    print("\n" + "=" * 107)
    print("Benchmark 1: Conditional Age Summation (process_*_users)")
    print("=" * 107)
    print("\n  Logic: For each user, if active=True, add age to total")
    print("  This prevents compiler optimization and forces actual data access")
    
    # Pydantic version - uses getattr with PyResult
    print("\n  Running process_pydantic_users...")
    result_pydantic = benchmark_function(
        py_rust_module.process_pydantic_users,
        pydantic_users,
        name="process_pydantic_users (getattr + PyResult)"
    )
    results.append(result_pydantic)
    total_age, active_count, elapsed = result_pydantic["result"]
    print(f"    Result: total_age={total_age:,}, active_count={active_count:,}, elapsed={elapsed:.2f} μs")
    
    # PyO3 version - direct field access
    print("  Running process_pyo3_users...")
    result_pyo3 = benchmark_function(
        py_rust_module.process_pyo3_users,
        pyo3_users,
        name="process_pyo3_users (direct field access)"
    )
    results.append(result_pyo3)
    total_age, active_count, elapsed = result_pyo3["result"]
    print(f"    Result: total_age={total_age:,}, active_count={active_count:,}, elapsed={elapsed:.2f} μs")
    
    # =========================================================================
    # Benchmark 2: Using benchmark_* functions (dict return)
    # =========================================================================
    print("\n" + "=" * 107)
    print("Benchmark 2: Conditional Age Summation (benchmark_*_process)")
    print("=" * 107)
    
    # Pydantic version
    print("\n  Running benchmark_pydantic_process...")
    result_bench_pydantic = benchmark_function(
        py_rust_module.benchmark_pydantic_process,
        pydantic_users,
        name="benchmark_pydantic_process"
    )
    results.append(result_bench_pydantic)
    bench_result = result_bench_pydantic["result"]
    print(f"    Result: total_age={bench_result['total_age']:,}, active_count={bench_result['active_count']:,}")
    
    # PyO3 version
    print("  Running benchmark_pyo3_process...")
    result_bench_pyo3 = benchmark_function(
        py_rust_module.benchmark_pyo3_process,
        pyo3_users,
        name="benchmark_pyo3_process"
    )
    results.append(result_bench_pyo3)
    bench_result = result_bench_pyo3["result"]
    print(f"    Result: total_age={bench_result['total_age']:,}, active_count={bench_result['active_count']:,}")
    
    # =========================================================================
    # Print Results Table
    # =========================================================================
    print("\n")
    print_results_table(results)
    
    # =========================================================================
    # Border Tax Analysis
    # =========================================================================
    print("\n" + "=" * 107)
    print("BORDER TAX ANALYSIS")
    print("=" * 107)
    
    # Analysis for Benchmark 1
    pydantic_time = result_pydantic["mean"]
    pyo3_time = result_pyo3["mean"]
    print_border_tax_analysis(pydantic_time, pyo3_time, "process_*_users Functions")
    
    # Analysis for Benchmark 2
    pydantic_time = result_bench_pydantic["mean"]
    pyo3_time = result_bench_pyo3["mean"]
    print_border_tax_analysis(pydantic_time, pyo3_time, "benchmark_*_process Functions")
    
    # =========================================================================
    # Summary
    # =========================================================================
    print("=" * 107)
    print("SUMMARY")
    print("=" * 107)
    print("""
The "Border Tax" represents the performance penalty when:

1. ENTRY TAX (Python → Rust conversion):
   - process_pydantic_users: Vec<PyObject> - no conversion needed
   - process_pyo3_users: Vec<PyRef<User>> - requires type coercion
   
2. ACCESS TAX (Attribute lookup):
   - Pydantic: getattr() → string hash + dict lookup on Python heap
   - PyO3: Direct field access at fixed memory offset (e.g., User + 12 bytes)
   
3. ERROR TAX (PyResult handling):
   - Pydantic: Must handle potential AttributeError for every getattr
   - PyO3: No error handling needed - fields guaranteed to exist

The conditional summation logic prevents compiler optimization by:
- Actually using the retrieved values (not a no-op)
- Creating a data dependency that forces real memory access
- Requiring both the 'active' check AND 'age' retrieval
""")


if __name__ == "__main__":
    # Use fixed seed for reproducible benchmarks
    random.seed(42)
    main()
