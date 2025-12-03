# Technical Details: radix256Sort

[日本語 (Japanese)](TECHNICAL_DETAILS_jp.md) | [Back to README](README.md)

`radix256Sort` is a high-performance, stable Radix Sort implementation specialized for 32-bit integers (`u32`).
It deliberately sacrifices generality to focus entirely on CPU cache efficiency and minimizing allocation, thereby outperforming standard sorts (`std::slice::sort` / TimSort).

## Innovations & Features

Here are the 6 key innovations and features in the design of this algorithm:

### 1. Explicit 256-Base, 4-Pass Definition
We eliminated variable radixes and generic designs, fixing the structure to process `u32` in 4 passes of 8 bits each.
This allows the loop count to be determined at compile time, minimizing branch prediction failures and inducing powerful compiler optimizations (such as loop unrolling).

### 2. Localized Fixed Counting Array
The histogram (counting array) used in each pass is allocated as a fixed-length array (`[usize; 256]`) on the stack, not the heap.
This reduces the overhead of dynamic memory allocation (`malloc`/`free`) to zero and dramatically improves the L1 cache hit rate.

### 3. Double-Buffered Fully Stable Sort
To guarantee "stability"—a requirement for LSD (Least Significant Digit) Radix Sort—we use a double-buffer strategy with a buffer (`to`) of the same size as the input array (`from`).
By switching buffers solely through pointer swapping (`swap`), we minimize data copy costs while maintaining mathematically proven stability.

### 4. Fixed-Length Prefix Sum
The cumulative sum (Prefix Sum) calculation to determine the write position for each bucket is also performed in a fixed-length loop of 256 elements.
This size fits reliably within the L1 cache, hiding memory access latency and executing extremely fast. In general generic Radix Sorts, this can be a bottleneck, but this implementation resolves it through fixed sizing.

### 5. Compliance with Rust Ownership Model (Safe Rust)
It is implemented entirely without `unsafe` blocks, fully complying with Rust's ownership system and borrow checker.
By properly managing `Vec` and slice ownership, we let the compiler guarantee memory safety (such as Bounds Checks), while minimizing the cost of Bounds Checks through iterator optimization. This is extremely valuable for potential adoption into standard libraries.

### 6. Proof of "Raw Array Acceleration" via PyO3
In the Python binding, we use PyO3 to bring Rust's speed directly into the Python world.
Python lists are efficiently converted to Rust `Vec`s and processed natively. While direct memory access is possible for NumPy arrays (if implemented), this implementation serves as a prime educational example of the structural speed advantages of Rust over Python.

## Algorithm Principle (LSD Radix Sort)

1.  **Preparation**: Allocate a buffer of the same size as the input array.
2.  **Loop**: Repeat the following process 4 times (shift = 0, 8, 16, 24).
    *   **Count**: Scan the input array and count occurrences of each value for the current digit (8 bits).
    *   **Accumulate**: Calculate the prefix sum of counts to determine the write start position for each bucket.
    *   **Reorder**: Rescan the input array and write values to the buffer according to the calculated positions.
    *   **Swap**: Swap the roles of the input array and the buffer.
3.  **Completion**: After the final pass, the data resides in either the original array or the buffer depending on the swap order. This implementation is designed to correctly return the result regardless of where it ends up.
