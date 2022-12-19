fn foo(mut a: String) -> String {
    if !a.ends_with("s") {
        a += " rust_awesome";
    }
    a
}

fn main() {
    let a = foo("hello".to_owned());
    println!("{a}");
}


