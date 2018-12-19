use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            // here rust notices when self.head moves to next, and rust doesnt like
            // that we leave it partially initialized untill we give the value back.
            next: self.head,
        };

        self.head = Link::More(new_node);
    }
}
