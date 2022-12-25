# Result:

```
$ cargo add rayon
    Updating crates.io index
      Adding rayon v1.6.1 to dependencies.

$ cargo run

   Compiling cfg-if v1.0.0
   Compiling scopeguard v1.1.0
   Compiling either v1.8.0
   Compiling memoffset v0.7.1
   Compiling libc v0.2.139
   Compiling crossbeam-utils v0.8.14
   Compiling num_cpus v1.15.0
   Compiling crossbeam-epoch v0.9.13
   Compiling crossbeam-channel v0.5.6
   Compiling crossbeam-deque v0.8.2
   Compiling rayon-core v1.10.1
   Compiling rayon v1.6.1
   Compiling rayon_hello v0.1.0 (/Users/globalyoung/Documents/test/test/rust/my_rust_project/05_Concurrency_Parallelism_Rust/Parallelism/rayon_hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.97s
     Running `target/debug/rayon_hello`

thread id : ThreadId(8)
thread id : ThreadId(5)
thread id : ThreadId(7)
thread id : ThreadId(3)
rayon let's go !!

```
