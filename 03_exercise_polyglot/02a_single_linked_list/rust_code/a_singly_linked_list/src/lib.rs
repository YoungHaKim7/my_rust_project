enum Node {
    Empty,
    NonEmpty(u32, Box<Node>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let node = &Node::Empty;
        let list = Node::NonEmpty(1091, Box::new(Node::Empty));
    }
}
