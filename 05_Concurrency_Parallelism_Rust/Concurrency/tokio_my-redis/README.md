# Result :

```
$ cargo run
   Compiling cfg-if v1.0.0
   Compiling once_cell v1.16.0
   Compiling libc v0.2.139
   Compiling smallvec v1.10.0
   Compiling pin-project-lite v0.2.9
   Compiling num-traits v0.2.15
   Compiling regex-syntax v0.6.28
   Compiling lazy_static v1.4.0
   Compiling log v0.4.17
   Compiling scopeguard v1.1.0
   Compiling serde v1.0.151
   Compiling tracing-core v0.1.30
   Compiling lock_api v0.4.9
   Compiling core-foundation-sys v0.8.3
   Compiling parking_lot_core v0.9.5
   Compiling unicode-width v0.1.10
   Compiling ansi_term v0.12.1
   Compiling iana-time-zone v0.1.53
   Compiling textwrap v0.11.0
   Compiling parking_lot v0.12.1
   Compiling num-integer v0.1.45
   Compiling tracing v0.1.37
   Compiling atty v0.2.14
   Compiling num_cpus v1.15.0
   Compiling mio v0.8.5
   Compiling socket2 v0.4.7
   Compiling signal-hook-registry v1.4.0
   Compiling memchr v2.5.0
   Compiling futures-core v0.3.25
   Compiling itoa v1.0.5
   Compiling strsim v0.8.0
   Compiling bitflags v1.3.2
   Compiling regex-automata v0.1.10
   Compiling ryu v1.0.12
   Compiling bytes v1.3.0
   Compiling vec_map v0.8.2
   Compiling clap v2.34.0
   Compiling regex v1.7.0
   Compiling pin-project v1.0.12
   Compiling chrono v0.4.23
   Compiling tracing-log v0.1.3
   Compiling matchers v0.0.1
   Compiling tokio v1.23.0
   Compiling thread_local v1.1.4
   Compiling tracing-serde v0.1.3
   Compiling serde_json v1.0.91
   Compiling sharded-slab v0.1.4
   Compiling async-stream v0.3.3
   Compiling tracing-futures v0.2.5
   Compiling atoi v0.3.3
   Compiling structopt v0.3.26
   Compiling tracing-subscriber v0.2.25
   Compiling tokio-stream v0.1.11
   Compiling mini-redis v0.4.1
   Compiling tokio_my-redis v0.1.0 (/Users/globalyoung/Documents/test/test/rust/my_rust_project/05_Concurrency_Parallelism_Rust/Concurrency/tokio_my-redis)
    Finished dev [unoptimized + debuginfo] target(s) in 5.05s
     Running `target/debug/tokio_my-redis`
Error: Os { code: 61, kind: ConnectionRefused, message: "Connection refused" }

$ mini-redis-server
zsh: command not found: mini-redis-server

$ cargo install mini-redis
    Updating crates.io index
  Installing mini-redis v0.4.1
   Compiling proc-macro2 v1.0.49
   Compiling unicode-ident v1.0.6
   Compiling quote v1.0.23
   Compiling autocfg v1.1.0
   Compiling syn v1.0.107
   Compiling libc v0.2.139
   Compiling cfg-if v1.0.0
   Compiling version_check v0.9.4
   Compiling once_cell v1.16.0
   Compiling log v0.4.17
   Compiling proc-macro-error-attr v1.0.4
   Compiling num-traits v0.2.15
   Compiling lock_api v0.4.9
   Compiling tracing-core v0.1.30
   Compiling serde v1.0.151
   Compiling smallvec v1.10.0
   Compiling pin-project-lite v0.2.9
   Compiling core-foundation-sys v0.8.3
   Compiling parking_lot_core v0.9.5
   Compiling num-integer v0.1.45
   Compiling proc-macro-error v1.0.4
   Compiling memchr v2.5.0
   Compiling futures-core v0.3.25
   Compiling lazy_static v1.4.0
   Compiling scopeguard v1.1.0
   Compiling regex-syntax v0.6.28
   Compiling tokio v1.23.0
   Compiling unicode-width v0.1.10
   Compiling serde_json v1.0.91
   Compiling ansi_term v0.12.1
   Compiling unicode-segmentation v1.10.0
   Compiling textwrap v0.11.0
   Compiling heck v0.3.3
   Compiling parking_lot v0.12.1
   Compiling iana-time-zone v0.1.53
   Compiling regex-automata v0.1.10
   Compiling atty v0.2.14
   Compiling signal-hook-registry v1.4.0
   Compiling socket2 v0.4.7
   Compiling num_cpus v1.15.0
   Compiling mio v0.8.5
   Compiling ryu v1.0.12
   Compiling strsim v0.8.0
   Compiling vec_map v0.8.2
   Compiling bytes v1.3.0
   Compiling bitflags v1.3.2
   Compiling itoa v1.0.5
   Compiling clap v2.34.0
   Compiling tracing-serde v0.1.3
   Compiling matchers v0.0.1
   Compiling regex v1.7.0
   Compiling chrono v0.4.23
   Compiling sharded-slab v0.1.4
   Compiling tracing-attributes v0.1.23
   Compiling tokio-macros v1.8.2
   Compiling pin-project-internal v1.0.12
   Compiling tracing v0.1.37
   Compiling structopt-derive v0.4.18
   Compiling async-stream-impl v0.3.3
   Compiling pin-project v1.0.12
   Compiling tracing-log v0.1.3
   Compiling thread_local v1.1.4
   Compiling tracing-subscriber v0.2.25
   Compiling structopt v0.3.26
   Compiling async-stream v0.3.3
   Compiling tracing-futures v0.2.5
   Compiling atoi v0.3.3
   Compiling tokio-stream v0.1.11
   Compiling mini-redis v0.4.1
    Finished release [optimized] target(s) in 13.62s
  Installing /Users/globalyoung/.cargo/bin/mini-redis-cli
  Installing /Users/globalyoung/.cargo/bin/mini-redis-server
   Installed package `mini-redis v0.4.1` (executables `mini-redis-cli`, `mini-redis-server`)

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/tokio_my-redis`

got value from the server; result=Some(b"world tokio")

```
