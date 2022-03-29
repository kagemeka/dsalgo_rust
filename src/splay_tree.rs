pub(crate) struct Node<K, V> {
    pub(crate) key: K,
    pub(crate) value: V,
    pub(crate) left: Option<Box<Node<K, V>>>,
    pub(crate) right: Option<Box<Node<K, V>>>,
    pub(crate) size: usize,
}

impl<K: PartialOrd, V> Node<K, V> {
    pub(crate) fn new(key: K, value: V) -> Box<Node<K, V>> {
        Box::new(Node {
            key: key,
            value: value,
            left: None,
            right: None,
            size: 1,
        })
    }

    pub(crate) fn get_size(root: Option<&Box<Self>>) -> usize {
        if let Some(node) = root { node.size } else { 0 }
    }

    pub(crate) fn update(root: &mut Box<Self>) {
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

    pub(crate) fn splay(root: Option<Box<Self>>, key: &K) -> Option<Box<Self>> {
        if root.is_none() {
            return None;
        }
        let mut root = root.unwrap();
        if &root.key == key {
            return Some(root);
        }
        if key < &root.key {
            if root.left.is_none() {
                return Some(root);
            }
            let mut left = root.left.take().unwrap();
            if key < &left.key && left.left.is_some() {
                left.left = Self::splay(left.left, key);
                root.left = Some(left);
                root = Self::rotate_right(root);
            } else if key > &left.key && left.right.is_some() {
                left.right = Self::splay(left.right, key);
                left = Self::rotate_left(left);
                root.left = Some(left);
            }
            root = Self::rotate_right(root);
        } else {
            if root.right.is_none() {
                return Some(root);
            }
            let mut right = root.right.take().unwrap();
            if key < &right.key && right.left.is_some() {
                right.left = Self::splay(right.left, key);
                right = Self::rotate_right(right);
                root.right = Some(right);
            } else if key > &right.key && right.right.is_some() {
                right.right = Self::splay(right.right, key);
                root.right = Some(right);
                root = Self::rotate_left(root);
            }
            root = Self::rotate_left(root);
        }
        Some(root)
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
    // TODO: min, max, lower_bound, upper_bound, get_kth_node
    // TODO: insert, remove
    // TODO: merge, split
    // TODO: insert with split and merge
    // TODO: remove with split and merge
    // TODO: index_mut
}
