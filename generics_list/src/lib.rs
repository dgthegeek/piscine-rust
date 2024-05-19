#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let node = Node {
            value,
            next: self.head.take().map(Box::new)
        };
        self.head = Some(node);
    }

    pub fn pop(&mut self) {
        self.head = self.head.take().and_then(|node| node.next.map(|boxed| *boxed));
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            length += 1;
            current_node = node.next.as_ref().map(|boxed| &**boxed);
        }

        length
    }
}