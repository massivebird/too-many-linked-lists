use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

pub struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

impl<T> List<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self { head: Link::None }
    }

    /// Prepends an element to the existing list.
    /// I think this is synonymous with a `push_front`.
    #[must_use]
    pub fn prepend(&self, elem: T) -> Self {
        Self {
            head: Some(Rc::new(Node {
                value: elem,
                next: self.head.clone(),
            })),
        }
    }

    #[must_use]
    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|n| &n.value)
    }

    #[must_use]
    pub fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
}

// We're self-implementing Drop since we have lots of Box<Node>, which does NOT
// drop using tail recursion; each drop will create a new stack frame.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            // drop nodes until there is one owned by another list
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let list: List<i32> = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(5);
        let list = list.prepend(2);
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&5));

        let list = list.tail();
        assert_eq!(list.head(), None);

        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}
