import hciutils  # Importing the Rust module we created
import time
import random

# Generate a large dataset for testing
large_data = [random.randint(1, 100) for _ in range(1_000_000)]

# Test RustList with parallel processing
rust_list = hciutils.RustList()
rust_list.extend(large_data)

# Measure parallel counting in RustList
start_time = time.time()
rust_count = rust_list.count_parallel(50)
rust_count_time = time.time() - start_time
print(f"RustList count_parallel time: {rust_count_time:.6f} seconds, Result: {rust_count}")

# Measure parallel sorting in RustList
start_time = time.time()
rust_list.sort_parallel()
rust_sort_time = time.time() - start_time
print(f"RustList sort_parallel time: {rust_sort_time:.6f} seconds")

# Test Python list for comparison
python_list = large_data.copy()

# Measure counting in Python list
start_time = time.time()
python_count = python_list.count(50)
python_count_time = time.time() - start_time
print(f"Python list count time: {python_count_time:.6f} seconds, Result: {python_count}")

# Measure sorting in Python list
start_time = time.time()
python_list.sort()
python_sort_time = time.time() - start_time
print(f"Python list sort time: {python_sort_time:.6f} seconds")

# Summary of results
print("\n--- Performance Comparison ---")
print(f"Count Parallel (RustList) vs Count (Python list): {python_count_time / rust_count_time:.2f}x speedup")
print(f"Sort Parallel (RustList) vs Sort (Python list): {python_sort_time / rust_sort_time:.2f}x speedup")
