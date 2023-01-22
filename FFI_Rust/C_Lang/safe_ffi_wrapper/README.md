# Result:

```

$ cargo run
   Compiling safe_ffi_wrapper v0.1.0 (/Users/globalyoung/Documents/test
/test/test/my_rust_project/FFI_Rust/C_Lang/safe_ffi_wrapper)
warning: unused imports: `CStr`, `OsStr`
  --> src/main.rs:26:16
   |
26 | use std::ffi::{CStr, CString, OsStr, OsString};
   |                ^^^^           ^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused import: `std::os::unix::ffi::OsStringExt`
  --> src/main.rs:27:5
   |
27 | use std::os::unix::ffi::OsStringExt;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused variable: `path`
  --> src/main.rs:36:12
   |
36 |     fn new(path: &str) -> Result<DirectoryIterator, String> {
   |            ^^^^ help: if this is intentional, prefix it with an un
derscore: `_path`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: fields `path` and `dir` are never read
  --> src/main.rs:31:5
   |
30 | struct DirectoryIterator {
   |        ----------------- fields in this struct
31 |     path: CString,
   |     ^^^^
32 |     dir: *mut ffi::DIR,
   |     ^^^
   |
   = note: `DirectoryIterator` has a derived impl for the trait `Debug`
, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` on by default

warning: struct `dirent` is never constructed
  --> src/main.rs:11:16
   |
11 |     pub struct dirent {
   |                ^^^^^^

warning: function `opendir` is never used
  --> src/main.rs:20:16
   |
20 |         pub fn opendir(s: *const c_char) -> *mut DIR;
   |                ^^^^^^^

warning: function `readdir` is never used
  --> src/main.rs:21:16
   |
21 |         pub fn readdir(s: *mut DIR) -> *const dirent;
   |                ^^^^^^^

warning: function `closedir` is never used
  --> src/main.rs:22:16
   |
22 |         pub fn closedir(s: *mut DIR) -> c_int;
   |                ^^^^^^^^

warning: `safe_ffi_wrapper` (bin "safe_ffi_wrapper") generated 8 warnin
gs
    Finished dev [unoptimized + debuginfo] target(s) in 2.43s
     Running `target/debug/safe_ffi_wrapper`
thread 'main' panicked at 'not implemented', src/main.rs:37:9
note: run with `RUST_BACKTRACE=1` environment variable to display a bac
ktrace
```
