# Result

```
fn compare_fibonaccis(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");

    group.bench_with_input("Recursive", &20, |b, i| b.iter(|| fibonacci_slow(*i)));
    group.bench_with_input("Iterative", &20, |b, i| b.iter(|| fibonacci_fast(*i)));
}
```

```
$ cargo criterion

   Compiling iterator_recursive v0.1.0 (/Users/globalyoung/Documents/test/test/rust/my_rust_project/06_Rust_Optimization/02_Iterative_Recursive/iterator_recursive)
    Finished bench [optimized] target(s) in 0.98s

fib 20                  time:   [19.791 µs 19.814 µs 19.843 µs]

fibonacci/Recursive     time:   [311.53 ps 312.63 ps 313.97 ps]
fibonacci/Iterative     time:   [6.8428 ns 6.8470 ns 6.8519 ns]

Fibonacci3/Recursive/20 time:   [311.08 ps 311.24 ps 311.41 ps]
Fibonacci3/Iterative/20 time:   [6.8439 ns 6.8483 ns 6.8535 ns]
Fibonacci3/Recursive/21 time:   [311.07 ps 311.21 ps 311.37 ps]
Fibonacci3/Iterative/21 time:   [7.1551 ns 7.1598 ns 7.1655 ns]

from_elem/1024          time:   [29.645 ns 29.696 ns 29.753 ns]
                        thrpt:  [32.053 GiB/s 32.114 GiB/s 32.170 GiB/s]
from_elem/2048          time:   [40.630 ns 40.662 ns 40.696 ns]
                        thrpt:  [46.868 GiB/s 46.907 GiB/s 46.944 GiB/s]
from_elem/4096          time:   [61.340 ns 62.137 ns 63.332 ns]
                        thrpt:  [60.234 GiB/s 61.392 GiB/s 62.190 GiB/s]
from_elem/8192          time:   [111.76 ns 115.23 ns 119.36 ns]
                        thrpt:  [63.920 GiB/s 66.207 GiB/s 68.264 GiB/s]
from_elem/16384         time:   [203.96 ns 211.22 ns 219.72 ns]
                        thrpt:  [69.446 GiB/s 72.240 GiB/s 74.813 GiB/s]

```
