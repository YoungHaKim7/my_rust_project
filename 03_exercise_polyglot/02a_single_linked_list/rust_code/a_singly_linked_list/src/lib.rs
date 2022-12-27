struct LinkedList {
    head: Link,
}

impl LinkedList {
    fn empty() -> LinkedList {
        LinkedList { head: None }
    }

    fn push(&mut self, element: u32) {
        let old_head = self.head.take();
        let new_head = Box::new(Node {
            element,
            next: old_head,
        });

        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<u32> {
        let old_head = self.head.take();
        old_head.map(|n| {
            self.head = n.next;
            n.element
        })
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
