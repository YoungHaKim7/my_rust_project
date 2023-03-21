use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut count = 10;
    print!("             <---   \\ | / - rotate : ");
    io::stdout().flush().unwrap(); // flush stdout to immediately show the countdown message

    while count > 0 {
        let a = "\\";
        print!("{} ", a);
        io::stdout().flush().unwrap(); // flush stdout to immediately show the numbers
        thread::sleep(Duration::from_millis(250)); // wait for 250 milliseconds before printing the next number
        print!("\r         \r"); // overwrite the previous number with spaces

        let b = "|";
        print!("{} ", b);
        io::stdout().flush().unwrap(); // flush stdout to immediately show the numbers
        thread::sleep(Duration::from_millis(250)); // wait for 250 milliseconds before printing the next number
        print!("\r         \r"); // overwrite the previous number with spaces

        let c = "/";
        print!("{} ", c);
        io::stdout().flush().unwrap(); // flush stdout to immediately show the numbers
        thread::sleep(Duration::from_millis(250)); // wait for 250 milliseconds before printing the next number
        print!("\r         \r"); // overwrite the previous number with spaces

        let c = "-";
        print!("{} ", c);
        io::stdout().flush().unwrap(); // flush stdout to immediately show the numbers
        thread::sleep(Duration::from_millis(250)); // wait for 250 milliseconds before printing the next number
        print!("\r         \r"); // overwrite the previous number with spaces
    }
    print!("{} ", count);
    io::stdout().flush().unwrap(); // flush stdout to immediately show the countdown number
    thread::sleep(Duration::from_millis(250)); // wait for 250 milliseconds before printing the next number
    print!("\r        \r"); // overwrite the previous number with spaces
    count -= 1;
}
