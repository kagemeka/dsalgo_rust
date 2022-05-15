use crate::avl_tree_node::Node;

#[derive(Debug)]
pub struct AVLTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: PartialOrd, V> AVLTree<K, V> {
    pub fn new() -> Self { Self { root: None } }

    pub fn size(&self) -> usize { Node::get_size(self.root.as_ref()) }

    pub fn insert(&mut self, key: K, value: V) {
        self.root = Some(Node::insert(
            self.root.take(),
            Node::new(key, value),
        ));
    }

    pub fn remove(&mut self, key: &K) {
        self.root = Node::remove(self.root.take(), &key);
    }

    pub fn get_kth_key(&self, k: usize) -> Option<&K> {
        if let Some(node) = Node::get_kth_node(self.root.as_ref(), k) {
            Some(&node.key)
        } else {
            None
        }
    }

    pub fn lower_bound(&self, key: &K) -> usize {
        Node::lower_bound(self.root.as_ref(), key)
    }

    pub fn upper_bound(&self, key: &K) -> usize {
        Node::upper_bound(self.root.as_ref(), key)
    }

    pub fn contains(&self, key: &K) -> bool {
        Node::find(self.root.as_ref(), key).is_some()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let mut tree = super::AVLTree::<&str, isize>::new();
        tree.insert("kagemeka", 1);
        tree.insert("kagemeka", 2);
        println!("{:?}", tree);
    }
}

// impl<K, V> Iterator<Item = &'a Box<Self>> for Node<K, V> {

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.left.is_some() {
//             return self.left.take().unwrap().next();
//         }
//         if self.right.is_some() {
//             return self.right.take().unwrap().next();
//         }
//         None
//     }
// }

// def iterate(root: typing.Optional[Node[K, V]]) ->
// typing.Iterator[Node[K, V]]:     def dfs(root:
// typing.Optional[Node[K, V]]) -> typing.Iterator[Node[K, V]]:
//         if root is None:
//             return
//         for node in dfs(root.left):
//             yield node
//         yield root
//         for node in dfs(root.right):
//             yield node

//     return dfs(root)
