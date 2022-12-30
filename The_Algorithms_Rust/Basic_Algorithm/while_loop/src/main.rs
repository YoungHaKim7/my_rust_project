// https://users.rust-lang.org/t/how-to-get-user-input/5176/2
// https://www.oreilly.com/library/view/programming-rust/9781491927274/ch15.html
use std::io;

fn sum_number(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n + 1 {
        sum += i;
    }
    sum
}

fn main() {
    println!("Calculates the sum of 1 to n.\n Please enter n no.: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("Calculates the sum of 1 to n : {} \n", sum_number(guess));
}
