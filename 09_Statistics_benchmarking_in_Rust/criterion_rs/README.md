# cargo criterion

```
$ cargo criterion
   Compiling criterion_rs v0.1.0 (/Users/globalyoung/Documents/test/test/rust/my_rust_project/09_Statistics_benchmarking_in_Rust/criterion_rs)
    Finished bench [optimized] target(s) in 0.80s
    
Benchmarking fib 20: Collecting 100 samples in estimated 5.0838 s
(258k iterat 

fib 20                  time:   [19.714 µs 19.724 µs 19.735 µs]


```

# cargo criterion

```
$ cargo criterion

   Compiling criterion_rs v0.1.0 (/Users/globalyoung/Documents/test/test/rust/my_rust_project/09_Statistics_benchmarking_in_Rust/criterion_rs)
    Finished bench [optimized] target(s) in 1.11s
    
fib 20                  time:   [19.715 µs 19.725 µs 19.736 µs]
                        change: [-0.0848% +0.0979% +0.2891%] (p = 0.31 > 0.05)
                        No change in performance detected.

Fibonacci/Recursive     time:   [311.25 ps 312.00 ps 312.90 ps]
Fibonacci/Iterative     time:   [6.8448 ns 6.8478 ns 6.8510 ns]

Fibonacci3/Recursive/20 time:   [311.34 ps 311.89 ps 312.57 ps]
Fibonacci3/Iterative/20 time:   [6.8395 ns 6.8427 ns 6.8463 ns]
Fibonacci3/Recursive/21 time:   [311.08 ps 311.25 ps 311.46 ps]
Fibonacci3/Iterative/21 time:   [7.1507 ns 7.1542 ns 7.1584 ns]

```

<hr>

<br>

# cargo-criterion

cargo-criterion is an experimental Cargo extension which can act as a replacement for cargo bench. The long-term goal for cargo-criterion is to handle all of the statistical analysis and report generation in a single tool. Then, the code for that can be removed from Criterion.rs (or made optional), reducing benchmark compilation and linking time. Since it manages the whole lifecycle of a benchmark run, cargo-criterion is also in a good position to provide features that would be difficult to implement in Criterion.rs itself.

Currently, cargo-criterion provides most of the same features as running Criterion.rs benchmarks in cargo bench, but with some differences:

cargo-criterion does not currently support baselines
cargo-criterion is more configurable than Criterion.rs
cargo-criterion supports machine-readable output using --message-format=json
cargo-criterion is stable, and you can install it with the following command:

```
cargo install cargo-criterion
```

Once installed, you can run your benchmarks with:


```
cargo criterion
```

If you encounter any issues or have any suggestions for future features, please raise an issue at the GitHub repository.
