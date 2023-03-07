# criterion.rs

- criterion.rs[![crates.io](https://img.shields.io/crates/v/criterion.svg)](https://crates.io/crates/criterion)![Crates.io](https://img.shields.io/crates/l/criterion)![druidDownloads](https://img.shields.io/crates/d/criterion.svg)<a href="[rustwasm/wasm-bindgen](https://github.com/bheisler/criterion.rs)"><img alt="githubicon" width="20px" src="https://user-images.githubusercontent.com/67513038/218287708-001511d7-1cce-42d3-92d2-4a61193b38f0.png" /></a>
![druidstar](https://img.shields.io/github/stars/bheisler/criterion.rs.svg)

  - Statistics-driven benchmarking library for Rust

    - https://github.com/bheisler/criterion.rs

    - https://crates.io/crates/criterion

<br>

# time 을 대체하는 command line 용 성능 측정 도구 hyperfine

https://www.lesstif.com/lpt/time-command-line-hyperfine-119963815.html

- hyperfine 은 커맨드 라인용 벤치마킹(benchmarking) 툴입니다.

  - https://github.com/sharkdp/hyperfine

<br>

# Rust_Optimizaion

https://gist.github.com/jFransham/369a86eff00e5f280ed25121454acec1

<br>

# The Rust Performance Book

https://nnethercote.github.io/perf-book/

# Let's Get Rusty

https://www.youtube.com/@letsgetrusty/

- What's up Rustaceans!

As Rust developers, I know many of you are always looking to give your code a performance boost.

Today I want to help you do that.

Here are some tips to help you optimize your code and squeeze out every last bit of performance!

1 . To improve performance, you must first understand the performance characteristics of your program. <a href="https://github.com/bheisler/criterion.rs">Criterion</a> is a statistics-driven microbenchmarking library that helps you write fast code by detecting and measuring performance improvements or regressions, even small ones, quickly and accurately.

2 . Use the <em style="color:red">#[inline] attribute</em> to hint to the compiler that a function should be inlined. This can help reduce the overhead of function calls, especially for small functions that are called frequently.

3. <em style="color:red">Avoid using the Box type</em> unless you need to allocate dynamically sized types on the heap. Prefer using stack-allocated types, such as arrays or tuples, whenever possible. For trait object, instead of Box you can typically get away with using &dyn Trait, which also has dynamic dispatch but saves an allocation.

4. <em style="color:red">Use Rust's const and static variables</em> to store values that do not change at runtime. Constants are evaluated at compile-time, while statics are stored in the binary and initialized at runtime. Using these variables can improve the performance of your program by allowing the compiler to optimize them more effectively.


# Installation

Clone the repo

```
git clone https://github.com/bheisler/criterion.rs.git
```

Change directories

```
cd criterion
```

Install dependencies and build app

```
cargo build
```

Run the benchmarks

```
cargo bench
    Finished release [optimized] target(s) in 0.09s
     Running target/release/deps/bencher-ef385ec0112399e3

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/my_benchmark-44ff04d7dba46a3e
fib 20                  time:   [17.262 us 17.425 us 17.587 us]
                        change: [-4.2772% -1.0604% +2.3046%] (p = 0.55 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high severe

```

View the plots

```
open target/criterion/report/index.html
```

![1](https://user-images.githubusercontent.com/67513038/223292095-13074f11-4373-41c1-9078-939e2b22b1c0.png)


https://github.com/cgcardona/bencher

