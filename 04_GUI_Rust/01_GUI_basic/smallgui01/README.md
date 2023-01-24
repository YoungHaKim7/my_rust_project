# Result

```

$ cargo run

   Compiling smallgui01 v0.1.0 (/Users/globalyoung/Documents/test/test/rust/my_rust_project/04_GUI_Rust/01_GUI_basic/smallgui01)
warning: unused variable: `window`
  --> src/main.rs:24:13
   |
24 |     let mut window = Window::new("Rust GUI Demo 1.23");
   |             ^^^^^^ help: if this is intentional, prefix it with an underscore: `_window`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variable does not need to be mutable
  --> src/main.rs:24:9
   |
24 |     let mut window = Window::new("Rust GUI Demo 1.23");
   |         ----^^^^^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: fields `title` and `widgets` are never read
  --> src/main.rs:10:5
   |
9  | struct Window {
   |        ------ fields in this struct
10 |     title: String,
   |     ^^^^^
11 |     widgets: Vec<Box<dyn Widget>>,
   |     ^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `smallgui01` (bin "smallgui01") generated 3 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `target/debug/smallgui01`

```
