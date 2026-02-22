#!/usr/bin/env python3
"""
Benchmark comparing Rust py_rust_module.User vs Pydantic User model
for JSON serialization/deserialization performance.
"""
import time
import statistics
from pydantic_model import User as PydanticUser

try:
    import py_rust_module
    RUST_AVAILABLE = True
except ImportError:
    RUST_AVAILABLE = False
    print("Warning: py_rust_module not available, skipping Rust benchmarks")


# Sample user data
USER_DATA = {
    "id": 1,
    "name": "Alice Johnson",
    "email": "alice@example.com",
    "age": 30,
    "active": True,
}

JSON_STRING = '{"id":1,"name":"Alice Johnson","email":"alice@example.com","age":30,"active":true}'

# Number of iterations for benchmark
ITERATIONS = 100000


def benchmark(func, *args, iterations=ITERATIONS, name="", **kwargs):
    """Run a function multiple times and return timing statistics"""
    times = []
    for _ in range(iterations):
        start = time.perf_counter()
        func(*args, **kwargs)
        end = time.perf_counter()
        times.append((end - start) * 1_000_000)  # Convert to microseconds

    return {
        "name": name,
        "mean": statistics.mean(times),
        "median": statistics.median(times),
        "stdev": statistics.stdev(times),
        "min": min(times),
        "max": max(times),
    }


def print_results(results):
    """Print benchmark results in a formatted table"""
    print(f"\n{'Operation':<30} {'Mean (μs)':<12} {'Median (μs)':<12} {'Stdev':<10} {'Min (μs)':<10} {'Max (μs)':<10}")
    print("-" * 94)
    for r in results:
        print(f"{r['name']:<30} {r['mean']:<12.2f} {r['median']:<12.2f} {r['stdev']:<10.2f} {r['min']:<10.2f} {r['max']:<10.2f}")


def main():
    print("=" * 94)
    print("Benchmark: Rust py_rust_module.User vs Pydantic User")
    print(f"Iterations: {ITERATIONS:,}")
    print("=" * 94)

    results = []

    # Create instances
    if RUST_AVAILABLE:
        rust_user = py_rust_module.User(
            id=USER_DATA["id"],
            name=USER_DATA["name"],
            email=USER_DATA["email"],
            age=USER_DATA["age"],
            active=USER_DATA["active"],
        )
        print("\n✓ Rust User created")

    pydantic_user = PydanticUser(**USER_DATA)
    print("✓ Pydantic User created")

    # Benchmark: Serialize to JSON (compact)
    print("\n--- JSON Serialization (compact) ---")
    if RUST_AVAILABLE:
        result = benchmark(rust_user.json, name="Rust User.json()")
        results.append(result)
        print(f"  Rust: {result['mean']:.2f} μs (mean)")

    result = benchmark(pydantic_user.json, name="Pydantic User.json()")
    results.append(result)
    print(f"  Pydantic: {result['mean']:.2f} μs (mean)")

    # Benchmark: Serialize to JSON (pretty)
    print("\n--- JSON Serialization (pretty) ---")
    if RUST_AVAILABLE:
        result = benchmark(rust_user.json_pretty, name="Rust User.json_pretty()")
        results.append(result)
        print(f"  Rust: {result['mean']:.2f} μs (mean)")

    result = benchmark(pydantic_user.json_pretty, name="Pydantic User.json_pretty()")
    results.append(result)
    print(f"  Pydantic: {result['mean']:.2f} μs (mean)")

    # Benchmark: Deserialize from JSON
    print("\n--- JSON Deserialization ---")
    if RUST_AVAILABLE:
        result = benchmark(py_rust_module.User.from_json, JSON_STRING, name="Rust User.from_json()")
        results.append(result)
        print(f"  Rust: {result['mean']:.2f} μs (mean)")

    result = benchmark(PydanticUser.from_json, JSON_STRING, name="Pydantic User.from_json()")
    results.append(result)
    print(f"  Pydantic: {result['mean']:.2f} μs (mean)")

    # Benchmark: Convert to dict
    print("\n--- Convert to Dict ---")
    if RUST_AVAILABLE:
        result = benchmark(rust_user.dict, name="Rust User.dict()")
        results.append(result)
        print(f"  Rust: {result['mean']:.2f} μs (mean)")

    result = benchmark(pydantic_user.dict, name="Pydantic User.dict()")
    results.append(result)
    print(f"  Pydantic: {result['mean']:.2f} μs (mean)")

    # Benchmark: Copy with modifications
    print("\n--- Copy with Modifications ---")
    if RUST_AVAILABLE:
        result = benchmark(
            rust_user.model_copy,
            "Alice Smith",
            USER_DATA["email"],
            31,
            USER_DATA["active"],
            name="Rust User.model_copy()",
        )
        results.append(result)
        print(f"  Rust: {result['mean']:.2f} μs (mean)")

    result = benchmark(pydantic_user.model_copy, name="Pydantic User.model_copy()", age=31)
    results.append(result)
    print(f"  Pydantic: {result['mean']:.2f} μs (mean)")

    # Print summary table
    print("\n")
    print_results(results)

    # Calculate speedup
    print("\n" + "=" * 94)
    print("Speedup Summary (Rust vs Pydantic)")
    print("=" * 94)

    if RUST_AVAILABLE:
        rust_results = [r for r in results if r["name"].startswith("Rust")]
        pydantic_results = [r for r in results if r["name"].startswith("Pydantic")]

        for rust_r, pydantic_r in zip(rust_results, pydantic_results):
            rust_op = rust_r["name"].replace("Rust ", "")
            pydantic_op = pydantic_r["name"].replace("Pydantic ", "")

            if rust_op.split("(")[0] == pydantic_op.split("(")[0]:
                speedup = pydantic_r["mean"] / rust_r["mean"]
                print(f"  {rust_op}: {speedup:.2f}x faster")


if __name__ == "__main__":
    main()
