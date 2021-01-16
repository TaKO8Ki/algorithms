use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node { data, next }
    }
}

pub struct SingleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SingleLinkedList<T> {
    pub fn new() -> Self {
        SingleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut next = &self.head;

        let mut count = 0;
        while let Some(n) = next {
            next = &n.next;
            count += 1;
        }

        count
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node::new(_element, self.head.take())));
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            self.head = head.next;

            return Some(head.data);
        }

        None
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(head) = &self.head {
            return Some(&head.data);
        }

        None
    }

    pub fn rev(self) -> SingleLinkedList<T> {
        let mut reversed = SingleLinkedList::new();
        let mut c = self.head;
        while let Some(e) = c {
            reversed.push(e.data);
            c = e.next;
        }

        reversed
    }
}

impl<T> Default for SingleLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> FromIterator<T> for SingleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SingleLinkedList::new();

        for i in _iter {
            list.push(i);
        }

        list
    }
}

impl<T> Into<Vec<T>> for SingleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = vec![];
        let mut c = self.head;
        while let Some(e) = c {
            vec.push(e.data);
            c = e.next;
        }

        vec.reverse();

        vec
    }
}
