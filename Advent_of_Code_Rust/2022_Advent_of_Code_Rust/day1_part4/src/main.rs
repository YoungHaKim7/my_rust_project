fn main() {
    let input = read_input();
    println!("{input}");
}

fn read_input() -> String {
    let folder = "src/input.txt".to_string();
    std::fs::read_to_string(folder).expect("while reading src/input.txt")
}
