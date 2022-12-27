struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn empty() -> LinkedList {
        LinkedList { head: None }
    }

    fn push(&mut self, element: u32) {
        match &self.head {
            Node => {
                self.head = Some(Box::new(Node {
                    element,
                    next: None,
                }))
            }
            Some(_) => todo!(),
        }
    }
}

struct Node {
    element: u32,
    next: Link,
}

type Link = Option<Box<Node>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = LinkedList::empty();
        list.push(1024);
    }
}
