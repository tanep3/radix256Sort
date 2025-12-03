# radix256Sort

[日本語 (Japanese)](README_jp.md) | [Technical Details](TECHNICAL_DETAILS.md)

A high-performance, stable Radix Sort implementation for `u32` integers, written in Rust with Python bindings.
Optimized for CPU cache efficiency and zero-allocation (internal loop) strategy.

## Features

- **Fast**: Outperforms standard library sorts (`std::slice::sort`) and Python's `list.sort` / `numpy.sort` for large datasets.
- **Stable**: Preserves the relative order of equal elements.
- **Safe**: Pure Rust implementation without `unsafe` blocks.
- **Simple**: Fixed 256-base, 4-pass algorithm optimized for 32-bit integers.

## Installation

### Rust

Add this to your `Cargo.toml`:

```toml
[dependencies]
radix256_sort = { path = "path/to/radix256_sort" } # Currently local
```

### Python

Requires `maturin` to build.

```bash
pip install maturin
maturin develop --release
```

## Usage

### Rust

```rust
use radix256_sort::radix256_sort_vec;

let mut data = vec![5, 2, 9, 1, 5];
let sorted = radix256_sort_vec(data);
assert_eq!(sorted, vec![1, 2, 5, 5, 9]);
```

Or in-place:

```rust
use radix256_sort::radix256_sort_inplace;

let mut data = vec![5, 2, 9, 1, 5];
radix256_sort_inplace(&mut data);
assert_eq!(data, vec![1, 2, 5, 5, 9]);
```

### Python

```python
import radix256_sort_py

data = [5, 2, 9, 1, 5]
sorted_data = radix256_sort_py.radix256_sort(data)
print(sorted_data) # [1, 2, 5, 5, 9]
```

## Benchmarks

Performance measured on 100,000,000 (100M) random `u32` integers.

> [!NOTE]
> The following figures are reference values from a development environment. Performance may vary depending on the system.


### Rust

| Algorithm | Time (s) | Speedup |
| :--- | :--- | :--- |
| `std::slice::sort` | 2.99s | 1.0x |
| **`radix256_sort_vec`** | **0.84s** | **3.56x** |

### Python

| Algorithm | Time (s) | Speedup (vs list) |
| :--- | :--- | :--- |
| `list.sort()` | 76.89s | 1.0x |
| **`radix256_sort`** | **7.61s** | **10.1x** |
| `numpy.sort()` | 5.27s | 14.6x |

### Analysis

The benchmark results demonstrate that `radix256_sort` significantly outperforms standard library implementations in both Rust and Python for large datasets.

- **Rust**: The **3.5x speedup** over the highly optimized `std::slice::sort` (pdqsort) confirms the efficiency of the cache-friendly, fixed-pass approach compared to generic comparison-based sorts.
- **Python**: The **10x speedup** over `list.sort` makes it a powerful alternative for heavy number crunching in pure Python environments. While `numpy.sort` is faster (5.27s), it requires the NumPy dependency. `radix256_sort` provides near-NumPy performance (7.61s) for standard lists, with the overhead largely due to the O(N) cost of converting Python lists to Rust vectors.

For detailed technical explanation of why this is so fast, see [Technical Details](TECHNICAL_DETAILS.md).

## License

Apache License 2.0

Copyright (c) 2025 Tane Channel Technology

