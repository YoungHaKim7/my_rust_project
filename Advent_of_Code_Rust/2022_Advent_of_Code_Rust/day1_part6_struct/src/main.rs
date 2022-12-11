use color_eyre::eyre::Context;

fn read_input() -> color_eyre::Result<String> {
    let input = std::fs::read_to_string("src/input.txt").wrap_err("reading src/input.txt")?;
    Ok(input)
}

fn main() {
    color_eyre::install().unwrap();

    let input = read_input().unwrap();
    println!("{input}");
}
