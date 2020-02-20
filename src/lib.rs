#[allow(dead_code)]
mod queue;
#[allow(dead_code)]
mod singly_linked_list;
#[allow(dead_code)]
mod stack;

#[cfg(test)]
mod tests {
    use crate::queue::Queue;
    use crate::singly_linked_list::SinglyLinkedList;
    use crate::stack::Stack;

    #[test]
    fn test_stack() {
        let mut stack = Stack::<i32>::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.size(), 3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);

        assert_eq!(stack.size(), 0);

        stack.push(4);

        assert_eq!(stack.top(), Some(&4));
        assert_eq!(stack.top_mut(), Some(&mut 4));

        stack.clear();
        assert_eq!(stack.empty(), true);
    }

    #[test]
    fn test_queue() {
        let mut queue = Queue::<i32>::new();

        queue.push(1);
        queue.push(2);
        queue.push(3);

        assert_eq!(queue.size(), 3);

        assert_eq!(queue.pop(), Some(Ok(1)));
        assert_eq!(queue.pop(), Some(Ok(2)));
        assert_eq!(queue.pop(), Some(Ok(3)));
        assert_eq!(queue.pop(), None);

        assert_eq!(queue.size(), 0);

        queue.push(7);

        assert_eq!(*queue.front().unwrap(), 7);
        assert_eq!(*queue.back_mut().unwrap(), 7);

        queue.clear();
        assert_eq!(queue.empty(), true);
    }

    #[test]
    fn test_singly_linked_list() {
        let mut list = SinglyLinkedList::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.size(), 3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        assert_eq!(list.size(), 0);

        list.push_front(7);
        list.push_front(4);
        list.push_front(1);

        assert_eq!(*list.front().unwrap(), 1);

        list.insert_after(3, |val| *val == 1);
        let supposed_node = list.head.as_ref().and_then(|x| x.next.as_ref());
        if let Some(node) = supposed_node {
            assert_eq!(node.value, 3)
        }

        // erase_after
        // let query_value = 1;
        // let mut it = list.head.as_mut();
        // while let Some(node) = it {
        //     if let Some(ref mut query_node) = node.next {
        //         if query_node.value == query_value {
        //             node.next = query_node.next.take();
        //         }
        //     }
        //     it = node.next.as_mut();
        // }
        // let supposed_node = list.head.as_ref();
        // if let Some(node) = supposed_node {
        //     assert_eq!(node.value, 3);
        // }

        list.clear();
        assert_eq!(list.empty(), true);
    }
}