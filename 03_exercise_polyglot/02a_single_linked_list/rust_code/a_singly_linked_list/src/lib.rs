struct LinkedList {
    head: Link,
}

struct Node {
    element: u32,
    next: List,
}

enum List {
    Empty,
    NonEmpty(Box<Node>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = List::NonEmpty(Box::new(Node {
            element: 1024,
            next: List::Empty,
        }));
    }
}
