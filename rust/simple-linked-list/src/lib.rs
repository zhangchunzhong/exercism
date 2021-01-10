use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut p = self.head.as_ref();
        let mut len: usize = 0;
        while p.is_some() {
            len += 1;
            p = p.unwrap().next.as_ref();
        }
        len
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take()
        }));
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T> where T: Clone {
        let mut ret_val = SimpleLinkedList::new();
        match &self.head {
            Some(node) => {
                let mut current: &Node<T> = node;

                ret_val.push(current.data.clone());
                while current.next.is_some() {
                    current = current.next.as_ref().unwrap();
                    ret_val.push(current.data.clone());
                };
                ret_val
            }
            None => ret_val
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut c = SimpleLinkedList::new();
        for i in iter {
            c.push(i);
        }
        c
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        match self.head {
            None => vec![],
            Some(node) => {
                let mut v = Vec::new();
                let mut current: Node<T> = *node;
                v.push(current.data);
                while current.next.is_some() {
                    current = *current.next.unwrap();
                    v.push(current.data);
                }
                v.reverse();
                v
            }
        }
    }
}
