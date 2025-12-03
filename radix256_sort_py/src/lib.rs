use pyo3::prelude::*;
use radix256_sort::radix256_sort_vec;

/// Sorts a list of integers using Radix Sort (base 256).
#[pyfunction(name = "radix256_sort")]
fn sort(list: Vec<u32>) -> Vec<u32> {
    radix256_sort_vec(list)
}

#[pymodule]
fn radix256_sort_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sort, m)?)?;
    Ok(())
}
