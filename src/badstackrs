// A linked list can be defined as being in one of two states:
// + Empty, or
// + Consisting of a node followed by another list!
//
// At first, we might build a Node enum. But if we make an enum of variants
// Nil and Cons(T, Box<List<T>>), we're building entire nodes dedicated to demarcating the
// end of a list. Plus, there are some implications of nodes being on the stack AND the heap.
// This is no good! Ideally:
// + All nodes are allocated in the same place (uniform allocation)
// + The list tail does not allocate excessive memory (see: null ptr optimization)
//
// We can achieve this by representing _nodes_ and _node relationships_ separately!
// Also, creating a pub struct that wraps these two allows us to keep the other two
// private.

use std::mem;

// struct w single field -> zero cost abstraction!
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

// Represents the relationship between nodes.
// Takes advantage of null pointer optimization by having two variants: one with no associated
// data, and the other with one associated pointer.
#[derive(Debug)]
enum Link<T> {
    Nil,
    // Self cannot contain Self — the recursion makes it impossible for Rust to calculate
    // the memory allocation requirements of Self.
    // We require a layer of indirection, which is possible via `Box`!
    Cons(Box<Node<T>>),
}

impl<T> List<T> {
    #[must_use] // linter error if invoked without binding return value
    pub const fn new() -> Self {
        Self { head: Link::Nil }
    }

    pub fn push_front(&mut self, new_value: T) {
        let new_node = Node {
            value: new_value,
            // We can't just assign next to self.head — that would move the pointer out of
            // self.head, thus invalidating it, and Rust ain't letting that happen, even for a moment.
            // Luckily, we can access self.head via a cheeky mem::replace, which does not leave
            // self.head invalidated. We'll give self.head a dummy ptr for now, then reassign it
            // below.
            next: mem::replace(&mut self.head, Link::Nil),
        };

        self.head = Link::Cons(Box::new(new_node));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // We cannot `match self.head` because the arm Link::Cons(node) attempts to move a node out
        // of self, which is an illegal mutate since we have a mutable ref to self.
        //
        // Trying to `match &self.head` -> we cannot reassign self.head inside the match expr since
        // we're matching against an immutable ref.
        //
        // We CAN do another cheeky mem::replace to acquire self.head by value without invalidating
        // self.head as a ptr!
        match mem::replace(&mut self.head, Link::Nil) {
            Link::Nil => None,
            Link::Cons(node) => {
                self.head = node.next;
                Some(node.value)
            },
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

// Default Drop isn't fully tail recursive! Namely, Box<Node> must drop its Node before
// deallocating itself.
// To fix this, we change all links in the list to Nil to avoid recursive drops.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Nil);
        while let Link::Cons(mut boxed_node) = current_link {
            current_link = mem::replace(&mut boxed_node.next, Link::Nil)
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn pushing_pulling() {
        let mut list: List<i32> = List::new();
        list.push_front(5);
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), None);
        list.push_front(7);
        list.push_front(8);
        list.push_front(13);
        assert_eq!(list.pop_front(), Some(13));
        assert_eq!(list.pop_front(), Some(8));
        assert_eq!(list.pop_front(), Some(7));
        assert_eq!(list.pop_front(), None);
    }
}
