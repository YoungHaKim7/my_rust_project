# Result

```bash
$ cargo bench

   Compiling criterion_fibo_test v0.1.0 (/Users/globalyoung/Documents/test/test/rust/my_rust_project/06_02_Benchmark/03_criterion_rs/criterion_fibo_test)
    Finished bench [optimized] target(s) in 11.35s
     Running unittests src/lib.rs (target/release/deps/criterion_fibo_test-ddc9202aa4a7c940)

running 1 test
test tests::it_works ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/my_benchmark.rs (target/release/deps/my_benchmark-82136d8a83a46706)

Benchmarking fib 20:
        Collecting 100 samples in estimated 5.0929 
        sfib 20
        time:   [19.698 µs 19.705 µs 19.714 µs]

Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe

```
