pub struct Second {
    value: u64,
}

impl Second {
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

fn main() {
    let s = Second::new(42);
    println!("s.value() = {}", s.value());
}
