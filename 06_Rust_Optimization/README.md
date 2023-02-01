# The Rust Performance Book

https://nnethercote.github.io/perf-book/

Here are some tips to help you optimize your code and squeeze out every last bit of performance!



1. To improve performance, you must first understand the performance characteristics of your program. Criterion is a statistics-driven microbenchmarking library that helps you write fast code by detecting and measuring performance improvements or regressions, even small ones, quickly and accurately.


2 .Use the #[inline] attribute to hint to the compiler that a function should be inlined. This can help reduce the overhead of function calls, especially for small functions that are called frequently.

3. Avoid using the Box type unless you need to allocate dynamically sized types on the heap. Prefer using stack-allocated types, such as arrays or tuples, whenever possible. For trait object, instead of Box you can typically get away with using &dyn Trait, which also has dynamic dispatch but saves an allocation.


4. Use Rust's const and static variables to store values that do not change at runtime. Constants are evaluated at compile-time, while statics are stored in the binary and initialized at runtime. Using these variables can improve the performance of your program by allowing the compiler to optimize them more effectively.


<br>


<hr>

# Rust Optimization 

https://gist.github.com/kvark/f067ba974446f7c5ce5bd544fe370186

<br>

<hr>

# How Rust Views Tradeoffs

https://youtu.be/2ajos-0OWts

<br>
