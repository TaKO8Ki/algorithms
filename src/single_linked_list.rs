use std::iter::FromIterator;

#[derive(Debug, PartialEq)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T, next: Option<Box<Node<T>>>) -> Self {
        Node { data, next }
    }
}

#[derive(Debug, PartialEq)]
pub struct SingleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SingleLinkedList<T> {
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
        let mut reversed = SingleLinkedList::default();
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
        SingleLinkedList { head: None }
    }
}

#[cfg(test)]
mod test {
    use super::{Node, SingleLinkedList};

    #[test]
    fn test_push() {
        let mut linked_list = SingleLinkedList::default();
        linked_list.push(1);
        assert_eq!(
            linked_list,
            SingleLinkedList {
                head: Some(Box::new(Node {
                    data: 1,
                    next: None
                }))
            }
        );
        linked_list.push(2);
        assert_eq!(
            linked_list,
            SingleLinkedList {
                head: Some(Box::new(Node {
                    data: 2,
                    next: Some(Box::new(Node {
                        data: 1,
                        next: None
                    }))
                }))
            }
        )
    }

    #[test]
    fn test_pop() {
        let mut linked_list = SingleLinkedList {
            head: Some(Box::new(Node {
                data: 2,
                next: Some(Box::new(Node {
                    data: 1,
                    next: None,
                })),
            })),
        };
        assert_eq!(linked_list.pop(), Some(2))
    }

    #[test]
    fn test_is_empty() {
        let linked_list1 = SingleLinkedList::<u8> { head: None };
        assert_eq!(linked_list1.is_empty(), true);
        let linked_list2 = SingleLinkedList {
            head: Some(Box::new(Node {
                data: 2,
                next: Some(Box::new(Node {
                    data: 1,
                    next: None,
                })),
            })),
        };
        assert_eq!(linked_list2.is_empty(), false)
    }

    #[test]
    fn test_len() {
        let linked_list = SingleLinkedList {
            head: Some(Box::new(Node {
                data: 2,
                next: Some(Box::new(Node {
                    data: 1,
                    next: None,
                })),
            })),
        };
        assert_eq!(linked_list.len(), 2)
    }
}
