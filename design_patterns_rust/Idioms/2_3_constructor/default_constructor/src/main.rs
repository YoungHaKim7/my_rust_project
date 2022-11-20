// https://rust-unofficial.github.io/patterns/idioms/ctor.html

pub struct Second {
    value: u64,
}

impl Second {
    pub fn value(&self) -> u64 {
        self.value
    }
}

impl Default for Second {
    fn default() -> Self {
        Self { value: 0 }
    }
}
fn main() {
    let s = Second::default();
    println!("s.value = {}", s.value());
}
