fn read_input() -> Result<String, std::io::Error> {
    std::fs::read_to_string("src/input.txt")
}

fn main() {
    let input = read_input().unwrap();
    println!("{input}");
}
