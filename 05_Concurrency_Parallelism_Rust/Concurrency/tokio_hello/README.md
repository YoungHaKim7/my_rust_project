# Result :

```
$ cargo add tokio -F tokio/full
    Updating crates.io index
      Adding tokio v1.23.0 to dependencies.
             Features:
             + bytes
             + fs
             + full
             + io-std
             + io-util
             + libc
             + macros
             + memchr
             + net
             + num_cpus
             + parking_lot
             + process
             + rt
             + rt-multi-thread
             + signal
             + signal-hook-registry
             + socket2
             + sync
             + time
             + tokio-macros
             - mio
             - stats
             - test-util
             - tracing
             - windows-sys

$ cargo run
    Blocking waiting for file lock on build directory
   Compiling cfg-if v1.0.0
   Compiling libc v0.2.139
   Compiling scopeguard v1.1.0
   Compiling smallvec v1.10.0
   Compiling bytes v1.3.0
   Compiling memchr v2.5.0
   Compiling pin-project-lite v0.2.9
   Compiling log v0.4.17
   Compiling lock_api v0.4.9
   Compiling parking_lot_core v0.9.5
   Compiling signal-hook-registry v1.4.0
   Compiling socket2 v0.4.7
   Compiling num_cpus v1.15.0
   Compiling mio v0.8.5
   Compiling parking_lot v0.12.1
   Compiling tokio v1.23.0
   Compiling tokio_hello v0.1.0 (/Users/globalyoung/Documents/test/test/rust/my_rust_project/05_Concurrency_Parallelism_Rust/Concurrency/tokio_hello)
error: the `async` keyword is missing from the function declaration
 --> src/main.rs:2:1
  |
2 | fn main() {
  | ^^

error: could not compile `tokio_hello` due to previous error


$ cargo run
   Compiling tokio_hello v0.1.0 (/Users/globalyoung/Documents/test/test/rust/my_rust_project/05_Concurrency_Parallelism_Rust/Concurrency/tokio_hello)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/tokio_hello`

Hello, world! tokio

```
