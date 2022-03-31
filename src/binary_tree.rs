use crate::{
    height::Height,
    insertion::Insert,
    join::Join,
    pop::Pop,
    size::Size,
    split::Split,
    tree_node::{Parent, ParentMut},
};

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;

        #[derive(Clone, Debug)]
        struct Node<T> {
            data: T,
            left: Option<Box<Self>>,
            right: Option<Box<Self>>,
        }

        struct Tree<V> {
            root: Option<V>,
        }

        impl<V> From<Option<Box<V>>> for Tree<V> {
            fn from(root: Option<Box<V>>) -> Self {
                if let Some(root) = root {
                    Tree {
                        root: Some(*root),
                    }
                } else {
                    Tree { root: None }
                }
            }
        }

        impl<T> Node<T> {
            fn new(data: T) -> Self {
                Node {
                    data,
                    left: None,
                    right: None,
                }
            }
        }

        impl<T> Size for Option<Node<T>>
        where
            T: Size,
        {
            fn size(&self) -> usize {
                match self {
                    Some(node) => node.data.size(),
                    None => 0,
                }
            }
        }

        impl<T: Size> Split<usize> for Option<Node<T>> {
            // pseudo
            fn split(mut self, index: usize) -> (Self, Self) {
                assert!(index <= self.size());
                (None, None)
            }
        }
        impl<T> Join for Option<Node<T>> {
            // pseudo
            fn join(self, rhs: Self) -> Self { None }
        }

        impl<T> Pop for Tree<T>
        where
            Option<T>: Split<usize> + Join + Size,
        {
            type Data = T;

            fn pop(&mut self, index: usize) -> Self::Data {
                assert!(self.root.is_some());
                let size = self.root.size();
                assert!(size > 0 && index < size);
                let (lhs, rhs) = self.root.take().split(index);
                let (popped, rhs) = rhs.split(1);
                self.root = lhs.join(rhs);
                popped.unwrap()
            }
        }

        impl<T> Insert for Tree<T>
        where
            Option<T>: Split<usize> + Join + Size,
        {
            type Data = T;

            fn insert(&mut self, index: usize, data: Self::Data) { let size = self.root.size(); }
        }
    }
}

// pub struct BinaryTree<K, V> {
//     key: K,
//     value: V,
//     left: Option<Box<Self>>,
//     right: Option<Box<Self>>,
//     // size: usize,
// }

// impl<K: PartialOrd, V> BinaryTree<K, V> {
//     pub fn get_size(&self) -> usize { 1 +
// self.left.get_size() + self.right.get_size() }

//     pub fn update(&mut self) { self.size = self.get_size();
// }

//     pub fn rotate_left(&mut self) {
//         assert!(self.right.left.is_some());
//         let mut new_root = self.right.left.take().unwrap();
//         assert!(self.right.left.is_none());
//         self.right.left = new_root.right.take();
//         self.update();
//         new_root.right = Some(self);
//         self.update();
//         *self = new_root;
//     }

//     pub fn rotate_right(&mut self) {
//         assert!(self.left.right.is_some());
//         let mut new_root = self.left.right.take().unwrap();
//         assert!(self.left.right.is_none());
//         self.left.right = new_root.left.take();
//         self.update();
//         new_root.left = Some(self);
//         self.update();
//         *self = new_root;
//     }

//     // pub fn splay(root: Option<BinaryTree<K, V>>, key: &K)
// ->     // Option<BinaryTree<K, V>> {     if root.is_none() {
//     //         return None;
//     //     }
//     //     let mut root = root.unwrap();
//     //     if &root.key == key {
//     //         return Some(root);
//     //     }
//     //     if key < &root.key {
//     //         if root.left.is_none() {
//     //             return Some(root);
//     //         }
//     //     }
//     // }
// }

// pub trait BinaryTreeNode {}

// pub trait Childs {
//     fn left(&self) -> &Option<Box<Self>>;
//     fn right(&self) -> &Option<Box<Self>>;
// }

// pub trait Size {
//     fn size(&self) -> usize;
// }

// pub trait GetSize {
//     fn get_size(_: &Option<Box<Self>>) -> usize;
// }

// impl<K, V> Childs for Node<K, V> {
//     fn left(&self) -> &Option<Box<Self>> { &self.left }

//     fn right(&self) -> &Option<Box<Self>> { &self.right }
// }

// impl<K, V> Size for Node<K, V> {
//     fn size(&self) -> usize { self.size }
// }

// impl<K, V> GetSize for Node<K, V> {
//     fn get_size(root: &Option<Box<Self>>) -> usize {
//         if let Some(root) = root { root.size } else { 0 }
//     }
// }

// pub trait Update {
//     fn update(&mut self);
// }

// impl<K, V> Update for Node<K, V> {
//     fn update(&mut self) {
//         self.size = Self::get_size(&self.left) +
// Self::get_size(&self.right) + 1;     }
// }

// pub trait Rotation {
//     // fn rotate_left(root: Box<Self>) -> Box<Self>;
//     // fn rotate_right(root: Self) -> Self>;
//     fn rotate_left(self) -> Box<Self>;
//     fn rotate_right(self) -> Box<Self>;
// }

// impl<T: Childs> Rotation for T {
//     fn rotate_left(mut self) -> Box<Self> {
//         assert!(self.right().is_some());
//         let mut new_root = self.right().take().unwrap();
//         assert!(self.right().is_none());
//         self.right() = new_root.left().take();
//         self.update();
//         new_root.left = Some(Box::new(self));
//         new_root.update();
//         new_root
//     }

//     fn rotate_right(mut self) -> Box<Self> {
//         let mut new_root = self.left.take().unwrap();
//         self.left = new_root.right.take();
//         self.update();
//         new_root.right = Some(Box::new(self));
//         new_root.update();
//         new_root
//     }
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn test() { let mut node = super::BinaryTree::new(1, 1);
// } }
