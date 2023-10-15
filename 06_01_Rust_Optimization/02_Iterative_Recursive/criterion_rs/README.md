# Result

```
$ cargo bench
     Running unittests src/main.rs (target/release/deps/criterion_rs-b9c0abed7a40e73f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/my_benchmark.rs (target/release/deps/my_benchmark-e5db99223b9bee19)

Benchmarking fib 20: Collecting 100 samples in estimated 5.0760 s (258fib 20                  time:   [19.725 µs 19.770 µs 19.825 µs]
Found 9 outliers among 100 measurements (9.00%)

  1 (1.00%) high mild
  8 (8.00%) high severe
```
