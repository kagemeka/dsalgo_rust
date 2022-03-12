#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) struct Node<K, V> {
    pub key: K,
    pub value: V,
    pub left: Option<Box<Node<K, V>>>,
    pub right: Option<Box<Node<K, V>>>,
    pub height: usize,
    pub size: usize,
}

impl<K: PartialOrd, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Box<Self> {
        Box::new(Self {
            key,
            value,
            left: None,
            right: None,
            height: 1,
            size: 1,
        })
    }

    pub(crate) fn get_height(root: Option<&Box<Self>>) -> usize {
        if let Some(node) = root {
            node.height
        } else {
            0
        }
    }

    pub(crate) fn get_size(root: Option<&Box<Self>>) -> usize {
        if let Some(node) = root { node.size } else { 0 }
    }

    pub(crate) fn get_balance(root: Option<&Box<Self>>) -> isize {
        if let Some(node) = root {
            Self::get_height(node.right.as_ref()) as isize
                - Self::get_height(node.left.as_ref()) as isize
        } else {
            0
        }
    }

    pub(crate) fn update(root: &mut Box<Self>) {
        // use std::borrow::BorrowMut;
        root.height = std::cmp::max(
            Self::get_height(root.left.as_ref()),
            Self::get_height(root.right.as_ref()),
        ) + 1;
        root.size = Self::get_size(root.left.as_ref()) + Self::get_size(root.right.as_ref()) + 1;
    }

    pub(crate) fn rotate_left(mut root: Box<Self>) -> Box<Self> {
        assert!(root.right.is_some());
        let mut new_root = root.right.take().unwrap();
        assert!(root.right.is_none());
        root.right = new_root.left.take();
        Self::update(&mut root);
        new_root.left = Some(root);
        Self::update(&mut new_root);
        new_root
    }

    pub(crate) fn rotate_right(mut root: Box<Self>) -> Box<Self> {
        assert!(root.left.is_some());
        let mut new_root = root.left.take().unwrap();
        assert!(root.left.is_none());
        root.left = new_root.right.take();
        Self::update(&mut root);
        new_root.right = Some(root);
        Self::update(&mut new_root);
        new_root
    }

    pub(crate) fn balance_tree(mut root: Box<Self>) -> Box<Self> {
        Self::update(&mut root);
        let balance = Self::get_balance(Some(&root));
        if balance < -1 {
            // lean left
            if Self::get_balance(root.left.as_ref()) > 0 {
                root.left = Some(Self::rotate_left(root.left.take().unwrap()));
            }
            return Self::rotate_right(root);
        } else if balance > 1 {
            // lean right
            if Self::get_balance(root.right.as_ref()) < 0 {
                root.right = Some(Self::rotate_right(root.right.take().unwrap()));
            }
            return Self::rotate_left(root);
        }
        root
    }

    /// return (popped, new_root)
    pub(crate) fn pop_max_node(mut root: Box<Self>) -> (Box<Self>, Option<Box<Self>>) {
        if root.right.is_none() {
            let new_root = root.left.take();
            root.left = None;
            return (root, new_root);
        }
        let (max_node, new_right) = Self::pop_max_node(root.right.take().unwrap());
        root.right = new_right;
        (max_node, Some(Self::balance_tree(root)))
    }

    pub fn insert(root: Option<Box<Self>>, node: Box<Self>) -> Box<Self> {
        if let Some(mut root) = root {
            if node.key <= root.key {
                root.left = Some(Self::insert(root.left.take(), node));
            } else {
                root.right = Some(Self::insert(root.right.take(), node));
            }
            Self::balance_tree(root)
        } else {
            node
        }
    }

    pub fn remove(root: Option<Box<Self>>, key: &K) -> Option<Box<Self>> {
        if root.is_none() {
            return None;
        }
        let mut root = root.unwrap();
        if key < &root.key {
            root.left = Self::remove(root.left.take(), key);
        } else if key > &root.key {
            root.right = Self::remove(root.right.take(), key);
        } else {
            if root.left.is_none() {
                return root.right;
            }
            let (max_node, new_left) = Self::pop_max_node(root.left.take().unwrap());
            root.left = new_left;
            root.key = max_node.key;
            root.value = max_node.value;
        }
        Some(Self::balance_tree(root))
    }

    pub fn get_kth_node(root: Option<&Box<Self>>, k: usize) -> Option<&Box<Self>> {
        if root.is_none() {
            return None;
        }
        let root = root.unwrap();
        let current_index = Self::get_size(root.left.as_ref());
        if k == current_index {
            return Some(root);
        }
        if k < current_index {
            Self::get_kth_node(root.left.as_ref(), k)
        } else {
            Self::get_kth_node(root.right.as_ref(), k - current_index - 1)
        }
    }

    pub fn lower_bound(root: Option<&Box<Self>>, key: &K) -> usize {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        if key <= &root.key {
            Self::lower_bound(root.left.as_ref(), key)
        } else {
            Self::get_size(root.left.as_ref()) + 1 + Self::lower_bound(root.right.as_ref(), key)
        }
    }

    pub fn upper_bound(root: Option<&Box<Self>>, key: &K) -> usize {
        if root.is_none() {
            return 0;
        }
        let root = root.unwrap();
        if key < &root.key {
            Self::upper_bound(root.left.as_ref(), key)
        } else {
            Self::get_size(root.left.as_ref()) + 1 + Self::upper_bound(root.right.as_ref(), key)
        }
    }

    pub fn find<'a>(root: Option<&'a Box<Self>>, key: &K) -> Option<&'a Box<Self>> {
        if root.is_none() {
            return None;
        }
        let root = root.unwrap();
        if key == &root.key {
            return Some(root);
        } else if key < &root.key {
            Self::find(root.left.as_ref(), key)
        } else {
            Self::find(root.right.as_ref(), key)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        type Node = super::Node<usize, usize>;
        let mut root = Some(Node::new(1, 1));
        println!("{:?}", root);
        root = Some(Node::insert(root, Node::new(2, 2)));
        println!("{:?}", root);
        root = Some(Node::insert(root, Node::new(3, 3)));
        println!("{:?}", root);
        root = Node::remove(root, &2);
        println!("{:?}", root);
        root = Node::remove(root, &2);
        println!("{:?}", root);
        assert_eq!(&Node::get_kth_node(root.as_ref(), 1).unwrap().key, &3);
        assert_eq!(Node::get_kth_node(root.as_ref(), 2), None);

        root = Some(Node::insert(root, Node::new(1, 2)));

        assert_eq!(Node::lower_bound(root.as_ref(), &1), 0);
        assert_eq!(Node::lower_bound(root.as_ref(), &2), 2);
        assert_eq!(Node::upper_bound(root.as_ref(), &1), 2);
        assert!(Node::find(root.as_ref(), &3).is_some());
        assert!(Node::find(root.as_ref(), &2).is_none());

        assert_eq!(Node::new(1, 1), Node::new(1, 1));
        assert!(!std::ptr::eq(&Node::new(1, 1), &Node::new(1, 1)));
    }
}
