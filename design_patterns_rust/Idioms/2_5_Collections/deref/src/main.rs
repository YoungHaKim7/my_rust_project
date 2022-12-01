use std::ops::Deref;

struct Vec<T> {
    data: RawVec<T>,
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {}
}

fn main() {
    println!("Hello, world!");
}
