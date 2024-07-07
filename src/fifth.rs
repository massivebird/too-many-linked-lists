// an OK unsafe queue

// tail is a ptr because if it were a Link, then both it and the head may try to
// own the same Node. That's no good, and we're tired of the Rc-RefCell
// solution. We're resorting to unsafety.
struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: std::ptr::null_mut(), // nullable mut ptr
        }
    }

    // push at the tail
    pub fn push(&mut self, new_elem: T) {
        let mut new_tail = Box::new(Node {
            elem: new_elem,
            next: None,
        });

        // Coercing a reference into a raw ptr!
        // This raw ptr points to the Box's heap contents.
        let raw_tail: *mut _ = &mut *new_tail;

        // before updating the list's tail...
        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }

        self.tail = raw_tail;
    }

    // pops front
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;

            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }

            head.elem
        })
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
