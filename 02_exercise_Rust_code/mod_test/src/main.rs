mod test;

use crate::test::Test::Young;

fn main() {
    let my_name = Young {
        name: "test".to_string(),
    };
    println!("Hello, world!{:?}", my_name);
}
