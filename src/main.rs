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

type Link<T> = Option<Box<Node<T>>>;

impl<T> List<T> {
    #[must_use] // linter error if invoked without binding return value
    pub const fn new() -> Self {
        Self { head: Link::None }
    }

    pub fn push_front(&mut self, new_value: T) {
        let new_node = Node {
            value: new_value,
            // We can't just assign next to self.head — that would move the pointer out of
            // self.head, thus invalidating it, and Rust ain't letting that happen, even for a moment.
            // Luckily, we can access self.head via a cheeky Option::take(), which does not leave
            // self.head invalidated. We'll give self.head a dummy ptr for now, then reassign it
            // below.
            next: self.head.take(),
        };

        self.head = Link::Some(Box::new(new_node));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // We cannot `match self.head` because the arm Link::Some(node) attempts to move a node out
        // of self, which is an illegal mutate since we have a mutable ref to self.
        //
        // Trying to `match &self.head` -> we cannot reassign self.head inside the match expr since
        // we're matching against an immutable ref.
        //
        // We CAN do another cheeky Option::take() to acquire self.head by value without invalidating
        // self.head as a ptr!
        match self.head.take() {
            Link::None => None,
            Link::Some(node) => {
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
// To fix this, we change all links in the list to None to avoid recursive drops.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Link::Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
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
