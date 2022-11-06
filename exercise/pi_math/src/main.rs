// https://rust-lang-nursery.github.io/rust-cookbook/hardware/processor.html

use num::complex::Complex;
use std::f64::consts::PI;

fn main() {
    let x = Complex::new(0.0, -2.0 * PI);

    println!("e^(-2i * pi) = {}", x.exp()); // =~1
}

// e^(-2i * pi) = 1+0.00000000000000024492935982947064i
// e^(1i * pi) = -1+0.00000000000000012246467991473532i
// e^(2i * pi) = -1-0.00000000000000024492935982947064i
// e^(3i * pi) = -1+0.00000000000000036739403974420594i
