#[derive(Default)]
pub struct Second {
    value: u64,
}

impl Second {
    pub fn value(&self) -> u64 {
        self.value
    }
}

fn main() {
    let s = Second::default();
    assert_eq!(0, s.value());
}
