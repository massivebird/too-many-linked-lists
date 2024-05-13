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
    pub fn new() -> Self {
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
            next: std::mem::replace(&mut self.head, Link::Nil),
        };

        self.head = Link::Cons(Box::new(new_node));
    }
}

fn main() {}
