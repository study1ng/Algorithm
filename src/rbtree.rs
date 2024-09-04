use std::collections::BTreeMap;

pub enum _RBTree<T> {
    Red{data: T, left: RBTree<T>, right: RBTree<T>},
    Black{data: T, left: RBTree<T>, right: RBTree<T>},
}

pub enum RBTree<T> {
    Nil,
    Node(Box<_RBTree<T>>)
}

impl<T> RBTree<T> {
    pub fn new() -> Self {
        RBTree::Nil
    }
    fn red(data: T) -> Self {
        RBTree::Node(Box::new(_RBTree::Red{data: data, left: RBTree::new(), right: RBTree::new()}))
    }
    fn black(data: T) -> Self {
        RBTree::Node(Box::new(_RBTree::Black{data: data, left: RBTree::new(), right: RBTree::new()}))
    }
    pub fn empty(&self) -> bool {
        match self {
            RBTree::Nil => true,
            _ => false,
        }
    }
    fn left(&self) -> &RBTree<T> {
        match self {
            RBTree::Node(node) => {
                match node.as_ref() {
                    _RBTree::Red{left, ..} => left,
                    _RBTree::Black{left, ..} => left,
                }
            },
            RBTree::Nil => panic!("Nil has no children"),
        }
    }
    fn right(&self) -> &RBTree<T> {
        match self {
            RBTree::Node(node) => {
                match node.as_ref() {
                    _RBTree::Red{right, ..} => right,
                    _RBTree::Black{right, ..} => right,
                }
            },
            RBTree::Nil => panic!("Nil has no children"),
        }
    }
}