// https://doc.rust-lang.org/book/ch12-05-working-with-environment-variables.html

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results =  Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    } 

    results
}

fn main() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    println!("search 'duct' : {:?}", search(query, contents));
}
