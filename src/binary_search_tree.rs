#[derive(Debug, PartialEq)]
pub struct BinarySearchTree<T>
where
    T: Ord,
{
    pub value: Option<T>,
    pub left: Option<Box<BinarySearchTree<T>>>,
    pub right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    pub fn insert(&mut self, value: T) {
        match &self.value {
            Some(v) => {
                if value < *v {
                    match &mut self.left {
                        Some(node) => node.insert(value),
                        None => {
                            let mut node = BinarySearchTree::default();
                            node.insert(value);
                            self.left = Some(Box::new(node))
                        }
                    }
                } else {
                    match &mut self.right {
                        Some(node) => node.insert(value),
                        None => {
                            let mut node = BinarySearchTree::default();
                            node.insert(value);
                            self.right = Some(Box::new(node))
                        }
                    }
                };
            }
            None => self.value = Some(value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    #[test]
    fn test_insert() {
        let mut root1 = BinarySearchTree {
            value: None,
            right: None,
            left: None,
        };
        root1.insert(1);
        assert_eq!(
            root1,
            BinarySearchTree {
                value: Some(1),
                right: None,
                left: None,
            }
        );

        let mut root2 = BinarySearchTree {
            value: Some(1),
            right: None,
            left: None,
        };
        root2.insert(3);
        assert_eq!(
            root2,
            BinarySearchTree {
                value: Some(1),
                right: Some(Box::new(BinarySearchTree {
                    value: Some(3),
                    right: None,
                    left: None
                })),
                left: None,
            }
        );

        let mut root3 = BinarySearchTree {
            value: Some(10),
            right: None,
            left: None,
        };
        root3.insert(9);
        assert_eq!(
            root3,
            BinarySearchTree {
                value: Some(10),
                right: None,
                left: Some(Box::new(BinarySearchTree {
                    value: Some(9),
                    right: None,
                    left: None
                })),
            }
        );

        let mut root4 = BinarySearchTree {
            value: Some(10),
            right: None,
            left: Some(Box::new(BinarySearchTree {
                value: Some(9),
                right: None,
                left: Some(Box::new(BinarySearchTree {
                    value: Some(7),
                    right: None,
                    left: None,
                })),
            })),
        };
        root4.insert(8);
        assert_eq!(
            root4,
            BinarySearchTree {
                value: Some(10),
                right: None,
                left: Some(Box::new(BinarySearchTree {
                    value: Some(9),
                    right: None,
                    left: Some(Box::new(BinarySearchTree {
                        value: Some(7),
                        right: Some(Box::new(BinarySearchTree {
                            value: Some(8),
                            right: None,
                            left: None
                        })),
                        left: None,
                    })),
                })),
            }
        )
    }
}
