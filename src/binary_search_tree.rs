use std::cmp::Ordering;

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
    pub fn search(&self, value: T) -> bool {
        match &self.value {
            Some(v) => match v.cmp(&value) {
                Ordering::Greater => match &self.left {
                    Some(node) => node.search(value),
                    None => false,
                },
                Ordering::Less => match &self.right {
                    Some(node) => node.search(value),
                    None => false,
                },
                Ordering::Equal => true,
            },
            None => false,
        }
    }

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

    pub fn min(&self) -> Option<&T> {
        match &self.value {
            Some(value) => match &self.left {
                Some(node) => node.min(),
                None => Some(value),
            },
            None => None,
        }
    }

    pub fn max(&self) -> Option<&T> {
        match &self.value {
            Some(value) => match &self.right {
                Some(node) => node.max(),
                None => Some(value),
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    #[test]
    fn test_search() {
        let root1 = BinarySearchTree {
            value: None,
            right: None,
            left: None,
        };
        assert_eq!(root1.search(1), false);

        let mut root2 = BinarySearchTree {
            value: None,
            right: None,
            left: None,
        };
        root2.insert(2);
        root2.insert(4);
        root2.insert(1);
        root2.insert(5);
        root2.insert(9);
        root2.insert(6);
        assert_eq!(root2.search(3), false);
        assert_eq!(root2.search(1), true);
    }

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
        );

        let mut root5 = BinarySearchTree {
            value: None,
            right: None,
            left: None,
        };
        root5.insert(2);
        root5.insert(4);
        root5.insert(1);
        root5.insert(5);
        root5.insert(9);
        root5.insert(6);
        assert_eq!(
            root5,
            BinarySearchTree {
                value: Some(2),
                right: Some(Box::new(BinarySearchTree {
                    value: Some(4),
                    right: Some(Box::new(BinarySearchTree {
                        value: Some(5),
                        right: Some(Box::new(BinarySearchTree {
                            value: Some(9),
                            right: None,
                            left: Some(Box::new(BinarySearchTree {
                                value: Some(6),
                                right: None,
                                left: None
                            }))
                        })),
                        left: None
                    })),
                    left: None
                })),
                left: Some(Box::new(BinarySearchTree {
                    value: Some(1),
                    right: None,
                    left: None
                })),
            }
        );
    }

    #[test]
    fn test_max() {
        assert_eq!(
            BinarySearchTree {
                value: Some(2),
                right: Some(Box::new(BinarySearchTree {
                    value: Some(4),
                    right: Some(Box::new(BinarySearchTree {
                        value: Some(5),
                        right: Some(Box::new(BinarySearchTree {
                            value: Some(9),
                            right: None,
                            left: Some(Box::new(BinarySearchTree {
                                value: Some(6),
                                right: None,
                                left: None
                            }))
                        })),
                        left: None
                    })),
                    left: None
                })),
                left: Some(Box::new(BinarySearchTree {
                    value: Some(1),
                    right: None,
                    left: None
                })),
            }
            .max(),
            Some(&9)
        )
    }

    #[test]
    fn test_min() {
        assert_eq!(
            BinarySearchTree {
                value: Some(2),
                right: Some(Box::new(BinarySearchTree {
                    value: Some(4),
                    right: Some(Box::new(BinarySearchTree {
                        value: Some(5),
                        right: Some(Box::new(BinarySearchTree {
                            value: Some(9),
                            right: None,
                            left: Some(Box::new(BinarySearchTree {
                                value: Some(6),
                                right: None,
                                left: None
                            }))
                        })),
                        left: None
                    })),
                    left: None
                })),
                left: Some(Box::new(BinarySearchTree {
                    value: Some(1),
                    right: None,
                    left: None
                })),
            }
            .min(),
            Some(&1)
        )
    }
}
