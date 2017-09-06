use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>, // DANGER DANGER
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None
        });

        let raw_tail: *mut _ = &mut *new_tail;

        // .is_null checks for null, equivalent to checking for None
        if !self.tail.is_null() {
            // If the old tail existed, update it to point to the new tail
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            // Otherwise, update the head to point to it
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }


    // pub fn pop(&'a mut self) -> Option<T> {
    //     // Grab the list's current head
    //     self.head.take().map(|head| {
    //         let head = *head;
    //         self.head = head.next;
    //
    //         // If we're out of `head`, make sure to set the tail to `None`.
    //         if self.head.is_none() {
    //             self.tail = None;
    //         }
    //
    //         head.elem
    //     })
    // }
}

mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }
}
