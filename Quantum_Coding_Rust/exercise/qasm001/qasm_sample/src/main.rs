extern crate qasm;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use qasm::{process, lex, parse};

fn main() {
    let cwd = env::current_dir().unwrap();
    let mut source = String::new();

    let mut f = File::open("test.qasm").expect("cannot find source file 'test.qasm'");
    f.read_to_string(&mut source).expect("couldn't read file 'test.qasm'");

    let processed_source = process(&source, &cwd);
    let mut tokens = lex(&processed_source);
    let ast = parse(&mut tokens);

    println!("{:?}", ast);
}
