use radix256_sort::radix256_sort_vec;
use rand::prelude::*;
use std::time::Instant;

fn main() {
    let size = 100_000_000;
    println!("Generating {} random u32 integers...", size);
    
    let mut rng = rand::thread_rng();
    let data: Vec<u32> = (0..size).map(|_| rng.gen()).collect();
    
    println!("Benchmarking Rust std::slice::sort...");
    let mut v1 = data.clone();
    let start = Instant::now();
    v1.sort();
    let duration = start.elapsed();
    println!("std::slice::sort: {:.4}s", duration.as_secs_f64());
    
    println!("Benchmarking radix256_sort_vec...");
    let v2 = data.clone();
    let start = Instant::now();
    let _sorted = radix256_sort_vec(v2);
    let duration = start.elapsed();
    println!("radix256_sort_vec: {:.4}s", duration.as_secs_f64());
}
