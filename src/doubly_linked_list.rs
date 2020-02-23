use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T, next: Link<T>, prev: Link<T>) -> Self {
        Self { value, next, prev }
    }
}

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    pub fn front_mut(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.value))
    }

    pub fn back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    pub fn back_mut(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.value))
    }

    pub fn push_front(&mut self, value: T) {
        self.size += 1;
        let new_head = Rc::new(RefCell::new(Node::new(value, None, None)));
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn push_back(&mut self, value: T) {
        self.size += 1;
        let new_tail = Rc::new(RefCell::new(Node::new(value, None, None)));
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(Rc::clone(&new_tail));
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<Result<T, &str>> {
        self.head.take().map(|old_head| {
            self.size -= 1;
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head)
                .map_err(|_| "The old head has >1 references")
                .map(|r| r.into_inner().value)
        })
    }

    pub fn pop_back(&mut self) -> Option<Result<T, &str>> {
        self.tail.take().map(|old_tail| {
            self.size -= 1;
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail)
                .map_err(|_| "The old tail has >1 references")
                .map(|r| r.into_inner().value)
        })
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}
