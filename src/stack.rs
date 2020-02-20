type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T, next: Link<T>) -> Self {
        Self { value, next }
    }
}

pub struct Stack<T> {
    head: Link<T>,
    size: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn top(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    pub fn push(&mut self, value: T) {
        self.size += 1;
        let new_node = Some(Box::new(Node::new(value, self.head.take())));
        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.size -= 1;
            self.head = old_head.next;
            old_head.value
        })
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut it = self.head.take();
        while let Some(node) = it {
            it = node.next;
        }
    }
}
