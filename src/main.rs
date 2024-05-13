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
        self.head.take().map(|boxed_node| {
            self.head = boxed_node.next;
            boxed_node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        // Option::map() wants to eat our self by value, which would move the Option out from under
        // it. We don't want to move anything; we just want a peek!
        // Option::as_ref() is the answer, which returns Option<&T> instead of Option<T>. Still a
        // little confused about this.
        self.head.as_ref().map(|boxed_node| &boxed_node.value)
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

// Tuple struct!
// We'll implement Iterator over this struct, which will consume the original list (moving it into
// an IntoIter instance), then "iterate" over its elements by consuming each one.
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        // still unsure as to what as_deref is doing here.
        Iter { next: self.head.as_deref() }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // still unsure as to what as_deref is doing here.
            self.next = node.next.as_deref();
            &node.value
        })
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

    #[test]
    fn peekaboo() {
        let mut list: List<i32> = List::new();
        list.push_front(5);
        list.push_front(2);
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.peek(), Some(&5));
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn into_itering() {
        let mut list: List<i32> = List::new();
        list.push_front(5);
        list.push_front(2);
        let mut lil_iter = list.into_iter();
        assert_eq!(lil_iter.next(), Some(2));
        assert_eq!(lil_iter.next(), Some(5));
        assert_eq!(lil_iter.next(), None);
    }
}
