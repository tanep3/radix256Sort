use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use radix256_sort::{radix256_sort_vec, radix256_sort_inplace};
use rand::prelude::*;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("radix256_sort");
    
    // Test sizes: 1M and 10M
    for size in [1_000_000, 10_000_000].iter() {
        let mut rng = rand::thread_rng();
        let data: Vec<u32> = (0..*size).map(|_| rng.gen()).collect();

        group.bench_with_input(BenchmarkId::new("vec", size), size, |b, &_size| {
            b.iter_with_setup(
                || data.clone(),
                |v| radix256_sort_vec(black_box(v))
            )
        });

        group.bench_with_input(BenchmarkId::new("inplace", size), size, |b, &_size| {
            b.iter_with_setup(
                || data.clone(),
                |mut v| radix256_sort_inplace(black_box(&mut v))
            )
        });
        
        group.bench_with_input(BenchmarkId::new("std_sort", size), size, |b, &_size| {
            b.iter_with_setup(
                || data.clone(),
                |mut v| v.sort()
            )
        });
        
        group.bench_with_input(BenchmarkId::new("std_sort_unstable", size), size, |b, &_size| {
            b.iter_with_setup(
                || data.clone(),
                |mut v| v.sort_unstable()
            )
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
