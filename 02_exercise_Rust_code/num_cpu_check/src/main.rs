// https://crates.io/crates/num_cpus
// https://rust-lang-nursery.github.io/rust-cookbook/hardware/processor.html

extern crate num_cpus;

fn main() {
    println!("Number of logical cores is {}", num_cpus::get());
}
