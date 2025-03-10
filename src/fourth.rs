use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> List<T> {
    #[must_use]
    const fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        if let Some(old_head) = self.head.take() {
            self.head = Some(new_head.clone());
            old_head.borrow_mut().prev = Some(new_head.clone());
            new_head.borrow_mut().next = Some(old_head);
        } else {
            self.head = Some(new_head.clone());
            self.tail = Some(new_head);
        }
    }

    fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);

        if let Some(old_tail) = self.tail.take() {
            self.tail = Some(new_tail.clone());
            old_tail.borrow_mut().next = Some(new_tail.clone());
            new_tail.borrow_mut().prev = Some(old_tail);
        } else {
            self.tail = Some(new_tail.clone());
            self.tail = Some(new_tail);
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    // list is emptied after this pop
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    // list is emptied after this pop
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    fn peek_front(&self) -> Option<Ref<T>> {
        // Returning Option<T> would be SO HARD with RefCells. RefCells produce
        // Ref[Mut]<'_, T>, which helps enforce runtime reference validation.
        // We can't access T without going through a Ref first.
        self.head
            .as_ref()
            // Our type is RefCell<Node<T>> — we don't want the Node part!
            // Ref::map allows us to convert Ref<T> -> Ref<F>.
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

// Inherits from Iterator (all DEIs are Iterators), also exposes the rev method
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

// We must self-implement drop to avoid reference cycles.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> Node<T> {
    #[must_use]
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem,
            next: None,
            prev: None,
        }))
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());

        list.push_front(5);
        list.push_front(3);
        list.push_front(2);

        assert_eq!(*list.peek_front().unwrap(), 2);
        assert_eq!(*list.peek_back().unwrap(), 5);
    }

    #[test]
    fn peek_mut() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());

        list.push_front(3);
        list.push_front(2);

        assert_eq!(*list.peek_front().unwrap(), 2);
        assert_eq!(*list.peek_back().unwrap(), 3);

        *list.peek_front_mut().unwrap() = 10;
        *list.peek_back_mut().unwrap() = 5;

        assert_eq!(*list.peek_front().unwrap(), 10);
        assert_eq!(*list.peek_back().unwrap(), 5);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut it = list.into_iter();

        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next_back(), Some(1));
        assert_eq!(it.next_back(), Some(2));
        assert_eq!(it.next(), None);
    }
}
