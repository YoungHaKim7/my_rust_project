use std::sync::Arc;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
    };

    let person2 = Person {
        name: "Gyoung".to_string(),
        age: 40,
    };

    let db = Arc::new([person, person2]);
    // let arc_person = Arc::new(person);

    for dbs in db.iter() {
        println!("db {dbs:?}");
    }
}
