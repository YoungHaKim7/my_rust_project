fn main() {
    let my_string = "Program";
    let g_index = my_string.find("g").unwrap(); // 3
    let g: String = my_string.chars().skip(g_index).take(1).collect();
    println!("{g}");
}
