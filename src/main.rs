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
