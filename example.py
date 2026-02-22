#!/usr/bin/env python3
"""
Example usage of the py_rust_module
"""
import json

try:
    import py_rust_module
    
    print("=== Testing Functions ===")
    result = py_rust_module.add(5, 3)
    print(f"add(5, 3) = {result}")
    
    result = py_rust_module.multiply(4, 7)
    print(f"multiply(4, 7) = {result}")
    
    greeting = py_rust_module.greet("Python Developer")
    print(f"greet('Python Developer') = {greeting}")
    
    print("\n=== Testing Calculator Class ===")
    calc = py_rust_module.Calculator(10.0)
    print(f"Initial calculator: {calc}")
    print(f"Initial value: {calc.value}")
    
    result = calc.add(5.0)
    print(f"After add(5.0): {result}")
    
    result = calc.multiply(2.0)
    print(f"After multiply(2.0): {result}")
    
    calc.reset()
    print(f"After reset(): {calc.value}")
    
    print("\n=== Testing User Model (Pydantic-like) ===")
    
    # Create a user instance
    user = py_rust_module.User(
        id=1,
        name="Alice Johnson",
        email="alice@example.com",
        age=30,
        active=True
    )
    print(f"User object: {user}")
    
    # Access properties
    print(f"User ID: {user.id}")
    print(f"User name: {user.name}")
    print(f"User email: {user.email}")
    
    # Serialize to JSON
    json_str = user.json()
    print(f"\nJSON output: {json_str}")
    
    # Pretty JSON
    json_pretty = user.json_pretty()
    print(f"\nPretty JSON:\n{json_pretty}")
    
    # Convert to dict
    user_dict = user.dict()
    print(f"\nAs dictionary: {user_dict}")
    
    # Deserialize from JSON
    user2 = py_rust_module.User.from_json(json_str)
    print(f"\nDeserialized user: {user2}")
    
    # Create a copy with modifications
    user3 = user.model_copy(name="Alice Smith", email=user.email, age=31, active=user.active)
    print(f"\nModified copy: {user3}")
    print(f"Original unchanged: {user}")
    
    print("\n✅ All tests passed!")
    
except ImportError as e:
    print("❌ Module not installed. Please build and install it first:")
    print("   pip install maturin")
    print("   maturin develop")
    print(f"\nError: {e}")