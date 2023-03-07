// https://bheisler.github.io/criterion.rs/book/user_guide/benchmarking_with_inputs.html

// https://github.com/bheisler/criterion.rs/tree/master/bencher_compat

use std::iter;

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        n => fibonacci_slow(n - 1) + fibonacci_slow(n - 2),
    }
}

fn fibonacci_fast(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn compare_fibonaccis(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");

    group.bench_with_input("Recursive", &20, |b, i| b.iter(|| fibonacci_slow(*i)));
    group.bench_with_input("Iterative", &20, |b, i| b.iter(|| fibonacci_fast(*i)));
}

fn compare_fibonaccis_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci3");
    for i in 20..=21 {
        group.bench_with_input(BenchmarkId::new("Recursive", i), &i, |b, i| {
            b.iter(|| fibonacci_slow(*i))
        });
        group.bench_with_input(BenchmarkId::new("Iterative", i), &i, |b, i| {
            b.iter(|| fibonacci_fast(*i));
        });
    }
    group.finish()
}

fn from_elem(c: &mut Criterion) {
    static KB: usize = 1024;

    let mut group = c.benchmark_group("from_elem");
    for size in [KB, 2 * KB, 4 * KB, 8 * KB, 16 * KB].iter() {
        group.throughput(Throughput::Bytes(*size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| iter::repeat(0u8).take(size).take(size).collect::<Vec<_>>());
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    criterion_benchmark,
    compare_fibonaccis,
    compare_fibonaccis_group,
    from_elem
);
criterion_main!(benches);
