// https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/linear_algebra.html
// invert matrix
use nalgebra::Matrix3;

fn main() {
    let m1 = Matrix3::new(2.0, 1.0, 1.0, 3.0, 2.0, 1.0, 2.0, 1.0, 2.0);
    println!("m1 = {}", m1);
    match m1.try_inverse() {
        Some(inv) => {
            println!("The inverse of m1 is: {}", inv);
        }
        None => {
            println!("m1 is not invertible!");
        }
    }
}

/*
 m1 =
  ┌       ┐
  │ 2 1 1 │
  │ 3 2 1 │
  │ 2 1 2 │
  └       ┘


The inverse of m1 is:
  ┌          ┐
  │  3 -1 -1 │
  │ -4  2  1 │
  │ -1  0  1 │
  └          ┘
*/
