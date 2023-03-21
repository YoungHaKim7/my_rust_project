use std::{
    io::{self, Write},
    thread, time,
};

fn main() {
    let start = time::Instant::now();
    let one_second = time::Duration::from_secs(1);
    let mut counter = 0;
    while counter <= 4 {
        thread::sleep(one_second);
        print!("ticking: {:.0}s", start.elapsed().as_secs_f32());
        counter += 1;
        std::process::Command::new("clear").status().unwrap();
        io::stdout().flush();
    }
}
