# Result

```

$ cargo run

warning: unused import: `std::fs::write`
 --> src/main.rs:4:5
  |
4 | use std::fs::write;
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `label`
  --> src/main.rs:95:17
   |
95 |         let mut label = self.width();
   |                 ^^^^^ help: if this is intentional, prefix it with an underscore: `_label`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variable does not need to be mutable
  --> src/main.rs:95:13
   |
95 |         let mut label = self.width();
   |             ----^^^^^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: field `callback` is never read
  --> src/main.rs:32:5
   |
30 | pub struct Button {
   |            ------ field in this struct
31 |     label: Label,
32 |     callback: Box<dyn FnMut()>,
   |     ^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `smallgui03` (bin "smallgui03") generated 4 warnings
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/smallgui03`
+--------------------------------+
|       Rust GUI Demo 1.23       |
+=============================== |
| This is a small text GUI Demo. |
| +-----------------+            |
| |    Click me!    +            |
| +-----------------+            |
+-                              -+
```
