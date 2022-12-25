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
   Compiling smallvec v1.10.0
   Compiling scopeguard v1.1.0
   Compiling pin-project-lite v0.2.9
   Compiling memchr v2.5.0
   Compiling bytes v1.3.0
   Compiling log v0.4.17
   Compiling lock_api v0.4.9
   Compiling parking_lot_core v0.9.5
   Compiling signal-hook-registry v1.4.0
   Compiling mio v0.8.5
   Compiling socket2 v0.4.7
   Compiling num_cpus v1.15.0
   Compiling parking_lot v0.12.1
   Compiling tokio v1.23.0
   Compiling tokio_hello2 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/my_rust_project/05_Concurrency_Parallelism_Rust/Concurrency/tokio_hello2)
    Finished dev [unoptimized + debuginfo] target(s) in 3.44s
     Running `target/debug/tokio_hello2`

Hello world tokio!!

```

https://tokio.rs/
