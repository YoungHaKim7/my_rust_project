# Source :

https://velog.io/@maxtnuk/%EC%A7%A4%EB%A7%89%ED%95%9C-Rust-%EC%83%9D%ED%99%9C%EA%B8%B0-%EC%A0%95%EC%A0%81-%EB%B3%80%EC%88%98-drop%EC%97%90-%EB%8C%80%ED%95%B4%EC%84%9C

<br>

<hr>

# Result:

```

$ cargo run

   Compiling c_drop_implement v0.1.0 (/Users/globalyoung/Documents/
test/test/rust/my_rust_project/FFI_Rust/C_Lang/c_drop_implement)
warning: static variable `__init_section` should have an upper casename

  --> src/main.rs:27:12
   |
27 | pub static __init_section: extern "C" fn() = __init_hello;
   |            ^^^^^^^^^^^^^^ help: convert the identifier to upper case: `__INIT_SECTION`
   |
   = note: `#[warn(non_upper_case_globals)]` on by default

warning: static variable `__fini_section` should have an upper casename
  --> src/main.rs:31:12
   |
31 | pub static __fini_section: extern "C" fn() = __fini_hello;
   |            ^^^^^^^^^^^^^^ help: convert the identifier to uppercase: `__FINI_SECTION`

warning: `c_drop_implement` (bin "c_drop_implement") generated 2 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/c_drop_implement`


init program
finish program
HELLO a: 0
main Hello, world! C ~~~ Best best
```

