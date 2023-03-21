use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    print!("Countdown: ");
    io::stdout().flush().unwrap();
    print!("\r                \r"); // overwrite the previous number with spaces
    loop {
        let count = ["1", "2", "3", "4"];
        print!("{}", count[0]);
        thread::sleep(Duration::from_millis(500)); // wait for 1 second before printing the next number
        print!("\r                \r"); // overwrite the previous number with spaces
        io::stdout().flush().unwrap(); // flush stdout to immediately show the countdown numbers
        print!("{} ", count[1]);
        io::stdout().flush().unwrap(); // flush stdout to immediately show the countdown numbers
        thread::sleep(Duration::from_millis(500)); // wait for 1 second before printing the next number
        print!("{} ", count[2]);
        io::stdout().flush().unwrap(); // flush stdout to immediately show the countdown numbers
        print!("\r                \r"); // overwrite the previous number with spaces
    }
}
