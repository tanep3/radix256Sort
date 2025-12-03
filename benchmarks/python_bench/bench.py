import time
import random
import sys
import os

# Ensure we can import the local build if needed, but typically we assume it's installed via maturin
try:
    import radix256_sort_py
except ImportError:
    print("Error: radix256_sort_py not found. Please install it with 'maturin develop --release'.")
    sys.exit(1)

try:
    import numpy as np
    HAS_NUMPY = True
except ImportError:
    HAS_NUMPY = False
    print("Warning: numpy not found. Skipping numpy benchmarks.")

def benchmark():
    size = 100_000_000
    print(f"Generating {size} random integers...")
    
    # Generate random data
    # Using numpy for fast generation if available, otherwise list comprehension (slow)
    if HAS_NUMPY:
        data_np = np.random.randint(0, 2**32, size, dtype=np.uint32)
        data_list = data_np.tolist()
    else:
        # This might take a while for 100M items
        data_list = [random.randint(0, 2**32 - 1) for _ in range(size)]

    print("Benchmarking Python list.sort()...")
    v1 = data_list.copy()
    start = time.time()
    v1.sort()
    end = time.time()
    print(f"list.sort(): {end - start:.4f}s")

    print("Benchmarking radix256_sort_py.radix256_sort...")
    v2 = data_list.copy()
    start = time.time()
    _sorted = radix256_sort_py.radix256_sort(v2)
    end = time.time()
    print(f"radix256_sort: {end - start:.4f}s")

    if HAS_NUMPY:
        print("Benchmarking numpy.sort()...")
        v3 = data_np.copy()
        start = time.time()
        _sorted_np = np.sort(v3)
        end = time.time()
        print(f"numpy.sort(): {end - start:.4f}s")

if __name__ == "__main__":
    benchmark()
