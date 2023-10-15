// https://bheisler.github.io/criterion.rs/book/user_guide/benchmarking_with_inputs.html
// https://github.com/YoungHaKim7/my_rust_project

use std::iter;
use std::mem::replace;

use criterion::{
    black_box, criterion_group, criterion_main, Bencher, BenchmarkId, Criterion, Throughput,
};

static BENCH_SIZE: usize = 20;

// recursive fibonacci
fn fibonacci(n: usize) -> u32 {
    if n < 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// iterative fibonacci
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);

        Some(replace(&mut self.curr, new_curr))
    }
}

fn fibonacci_sequence() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

fn recursive_fibonacci(b: &mut Bencher) {
    // exact code to benchmark must be passed as a closure to the iter
    // method of Bencher
    b.iter(|| (0..BENCH_SIZE).map(fibonacci).collect::<Vec<u32>>())
}

fn iterative_fibonacci(b: &mut Bencher) {
    b.iter(|| fibonacci_sequence().take(BENCH_SIZE).collect::<Vec<u32>>())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

fn compare_fibonaccis(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");

    group.bench_with_input("Recursive", &20, |b, i| b.iter(|| fibonacci(*i)));
    group.bench_with_input("Iterative", &20, |b, i| b.iter(|| fibonacci_sequence(*i)));
}

fn compare_fibonaccis_group(c: &mut Criterion) {
    let mut group = c.benchmark_group("Fibonacci3");
    for i in 20..=21 {
        group.bench_with_input(BenchmarkId::new("Recursive", i), &i, |b, i| {
            b.iter(|| fibonacci(*i))
        });
        group.bench_with_input(BenchmarkId::new("Iterative", i), &i, |b, i| {
            b.iter(|| fibonacci_sequence(*i));
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
