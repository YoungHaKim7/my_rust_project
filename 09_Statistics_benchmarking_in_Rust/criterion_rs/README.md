# cargo criterion

```
$ cargo criterion
   Compiling criterion_rs v0.1.0 (/Users/globalyoung/Documents/test/test/rust/my_rust_project/09_Statistics_benchmarking_in_Rust/criterion_rs)
    Finished bench [optimized] target(s) in 0.80s
    
Benchmarking fib 20: Collecting 100 samples in estimated 5.0838 s
(258k iterat 

fib 20                  time:   [19.714 µs 19.724 µs 19.735 µs]


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
