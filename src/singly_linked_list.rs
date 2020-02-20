type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    pub value: T,
    pub next: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T, next: Link<T>) -> Self {
        Self { value, next }
    }
}

pub struct SinglyLinkedList<T> {
    pub head: Link<T>,
    size: usize,
}

impl<T> SinglyLinkedList<T> {
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

    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    pub fn push_front(&mut self, value: T) {
        self.size += 1;
        let new_node = Some(Box::new(Node::new(value, self.head.take())));
        self.head = new_node;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.size -= 1;
            self.head = old_head.next;
            old_head.value
        })
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }

    pub fn insert_after(&mut self, new_value: T, predicate: impl Fn(&T) -> bool) {
        let mut it = self.head.as_mut();
        while let Some(node) = it {
            if predicate(&node.value) {
                let new_node = Some(Box::new(Node::new(new_value, node.next.take())));
                node.next = new_node;
                break;
            }
            it = node.next.as_mut();
        }
    }

    pub fn erase_after(&mut self, predicate: impl Fn(&T) -> bool) {
        let mut it = self.head.as_mut();
        while let Some(node) = it {
            if let Some(ref mut query_node) = node.next {
                if predicate(&query_node.value) {
                    node.next = query_node.next.take();
                }
            }
            it = node.next.as_mut();
        }
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        let mut it = self.head.take();
        while let Some(node) = it {
            it = node.next;
        }
    }
}
