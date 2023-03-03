use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus { like: true };
    println!("I'm growing {plant:?}");
}
