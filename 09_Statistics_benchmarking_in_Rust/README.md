# Statistics-driven benchmarking library for Rust

https://github.com/bheisler/criterion.rs

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

3. <em style="color:red">Avoidusing the Box type</em> unless you need to allocate dynamically sized types on the heap. Prefer using stack-allocated types, such as arrays or tuples, whenever possible. For trait object, instead of Box you can typically get away with using &dyn Trait, which also has dynamic dispatch but saves an allocation.

4. <em style="color:red">Use Rust's const and static variables</em> to store values that do not change at runtime. Constants are evaluated at compile-time, while statics are stored in the binary and initialized at runtime. Using these variables can improve the performance of your program by allowing the compiler to optimize them more effectively.
