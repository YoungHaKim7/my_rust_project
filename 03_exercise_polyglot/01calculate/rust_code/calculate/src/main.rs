fn calculate(bottom: i32, top: i32) -> i32 {
    (bottom..=top).filter(|e| e % 2 == 0).sum()
}

fn main() {
    println!("{}", calculate(5,12));
    println!("{}", calculate(5,3));
}
