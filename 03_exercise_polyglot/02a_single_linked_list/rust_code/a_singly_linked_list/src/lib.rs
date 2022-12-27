struct LinkedList {
    head: Link,
}

struct Node {
    element: u32,
    next: Link,
}

enum Link {
    Empty,
    NonEmpty(Box<Node>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = Link::NonEmpty(Box::new(Node {
            element: 1024,
            next: Link::Empty,
        }));
    }
}
