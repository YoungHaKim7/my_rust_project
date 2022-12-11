use std::fmt;

struct Payload {
  id: i32,
  value: i32,
}

impl fmt::Display for Payload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.value)
    }
}

struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> where T: std::fmt::Display{
    fn new(element: T) -> Self {
        Node {
            element: element,
            next: None,
        }
    }

    fn append(&mut self, element: T) {
        match &mut self.next {
            None => {let n = Node {
                        element: element,
                        next: None,
                    };
                    self.next = Some(Box::new(n));
            },
            Some(ref mut x) => x.append(element),
        }
    }

    fn list(& self) {
        println!("{}", self.element);
        match &self.next {
            None => {},
            Some(x) => x.list(),
        }
    }
}

fn main(){
  let mut h = Node::new(Payload {id:1, value:1});
  h.append(Payload {id:2, value:2});
  h.append(Payload {id:3, value:3});
  h.append(Payload {id:4, value:4});
  h.append(Payload {id:5, value:5});
  h.list();
  h.append(Payload {id:6, value:6});
  h.list();
}
