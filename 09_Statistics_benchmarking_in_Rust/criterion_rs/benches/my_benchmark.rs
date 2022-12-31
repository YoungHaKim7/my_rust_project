use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

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
    let mut group = c.benchmark_group("Fibonacci");

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
            b.iter(|| fibonacci_fast(*i))
        });
    }
    group.finish()
}

criterion_group!(
    benches,
    criterion_benchmark,
    compare_fibonaccis,
    compare_fibonaccis_group
);
criterion_main!(benches);
