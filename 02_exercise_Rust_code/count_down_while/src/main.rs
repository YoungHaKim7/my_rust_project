// fn main() {
//     let mut count = 10;
//     print!("Countdown: ");
//     while count > 0 {
//         print!("{} ", count);
//         count -= 1;
//     }
//     println!("Blast off!");
//
// }
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut count = 10;
    print!("Countdown: ");
    io::stdout().flush().unwrap(); // flush stdout to immediately show the countdown message

    while count > 0 {
        print!("{} ", count);
        io::stdout().flush().unwrap(); // flush stdout to immediately show the countdown numbers
        thread::sleep(Duration::from_secs(1)); // wait for 1 second before printing the next number
        print!("\r                \r"); // overwrite the previous number with spaces
        count -= 1;
    }

    println!("Blast off!");
}
