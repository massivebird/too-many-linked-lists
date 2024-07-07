use std::ptr;

// an OK unsafe queue

// tail is a ptr because if it were a Link, then both it and the head may try to
// own the same Node. That's no good, and we're tired of the Rc-RefCell
// solution. We're resorting to unsafety.
// Also, head is following suit. Mixing ptrs with refs is messy.
struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

// For our singly-linked queue, we can either:
//    + Push front and pop back, or
//    + Push back and pop front.
// Singly-linked means the back is more important. Popping the back would be
// messy: you would have to move the tail backwards, which requires a O(n)
// traversal. Instead, we'll push to the back, which moves the tail forwards
// at O(1).
impl<T> List<T> {
    pub const fn new() -> Self {
        Self {
            head: ptr::null_mut(), // nullable mut ptr
            tail: ptr::null_mut(),
        }
    }

    // push at the tail
    pub fn push(&mut self, new_elem: T) {
        unsafe {
            // create a Box and convert it into a raw ptr
            let new_tail = Box::into_raw(Box::new(Node {
                elem: new_elem,
                next: ptr::null_mut(),
            }));

            // before updating the list's tail...
            if self.tail.is_null() {
                self.head = new_tail;
            } else {
                (*self.tail).next = new_tail;
            }

            self.tail = new_tail;
        }
    }

    // pops front
    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                // list is currently empty
                None
            } else {
                // own the current head by turning it into a Box!
                // This also cleans up the data via the Box drop
                let old_head = Box::from_raw(self.head);

                self.head = old_head.next;

                // list is now emptied
                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(old_head.elem)
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        list.push(1);
        list.push(2);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);

        list.push(3);
        list.push(4);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), None);
    }
}
