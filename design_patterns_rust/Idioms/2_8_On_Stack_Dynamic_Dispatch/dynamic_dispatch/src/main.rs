// https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
// use std::io;

use std::fs;
use std::env;

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

fn main(){
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
    // let (mut stdin_read, mut file_read);

    // let arg = fs::read_to_string(file_path).expect("Should have been able to read the file");
    //  
    // let readable: &mut dyn io::Read = if arg == "-" {
    //     stdin_read = io::stdin();
    //     &mut stdin_read
    // } else {
    //     file_read = match fs::File::open(arg) {
    //         Ok(it) => it,
    //         Err(err) => return Err(err),
    //     };
    //     &mut file_read
    // };

    // Ok(())
}
