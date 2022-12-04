fn main() {
    let input = match std::fs::read_to_string("src/input.txt") {
        Ok(s) => s,
        Err(e) => panic!("Couldn't read src/input.txt: {e}"),
    };

    println!("{input}");
}
