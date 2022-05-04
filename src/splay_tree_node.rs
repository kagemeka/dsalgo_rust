use std::{cell::RefCell, rc::Rc};

use crate::{
    binary_tree_node,
    binary_tree_node::{Rotation, Update},
    join::Join,
    size,
    size::Size,
    split::Split,
    tree_node::Get,
};

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub parent: Option<Rc<RefCell<Self>>>,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Node {
            data,
            parent: None,
            left: None,
            right: None,
        }
    }
}

impl<T: size::Size> size::Size for Node<T> {
    fn size(&self) -> usize { self.data.size() }
}

impl<T: size::Size> size::Size for Option<Node<T>> {
    fn size(&self) -> usize { self.as_ref().map_or(0, |node| node.size()) }
}

impl<T: size::Size> size::Size for Rc<RefCell<Node<T>>> {
    fn size(&self) -> usize { self.borrow().size() }
}

impl<T: size::Size> size::Size for Option<Rc<RefCell<Node<T>>>> {
    fn size(&self) -> usize { self.as_ref().map_or(0, |node| node.size()) }
}

#[derive(PartialEq)]
enum State {
    LeftChild,
    RightChild,
    Root,
}

impl<T> Node<T>
where
    Node<T>: binary_tree_node::Update,
{
    fn rotation_common_ops(
        previous_root: &Rc<RefCell<Self>>,
        new_root: &Rc<RefCell<Self>>,
        child: &Option<Rc<RefCell<Self>>>,
    ) {
        previous_root.borrow_mut().update();
        new_root.borrow_mut().update();
        if let Some(child) = child {
            child.borrow_mut().parent = Some(previous_root.clone());
        }
        new_root.borrow_mut().parent = previous_root.borrow_mut().parent.take();
        if let Some(parent) = &new_root.borrow().parent {
            match Node::<T>::get_state(previous_root) {
                State::LeftChild => parent.borrow_mut().left = Some(new_root.clone()),
                State::RightChild => parent.borrow_mut().right = Some(new_root.clone()),
                _ => (),
            }
            parent.borrow_mut().update();
        }
        previous_root.borrow_mut().parent = Some(new_root.clone());
    }

    fn rotate_up(node: &Rc<RefCell<Self>>) {
        let parent = node.borrow().parent.as_ref().unwrap().clone();
        if parent.borrow().left.is_some()
            && Rc::ptr_eq(parent.borrow().left.as_ref().unwrap(), node)
        {
            parent.rotate_right();
        } else {
            assert!(Rc::ptr_eq(parent.borrow().right.as_ref().unwrap(), node));

            parent.rotate_left();
        }
    }

    fn get_state(node: &Rc<RefCell<Self>>) -> State {
        match &node.borrow().parent {
            Some(parent) => {
                if parent.borrow().left.is_some()
                    && Rc::ptr_eq(parent.borrow().left.as_ref().unwrap(), node)
                {
                    State::LeftChild
                } else {
                    State::RightChild
                }
            },
            None => State::Root,
        }
    }

    fn splay(node: &Rc<RefCell<Self>>) {
        loop {
            // don't use while let to avoid borrowing conflict.
            if node.borrow().parent.is_none() {
                break;
            }
            let parent = node.borrow().parent.as_ref().unwrap().clone();
            let node_state = Node::<T>::get_state(node);
            if parent.borrow().parent.is_some() {
                let parent_state = Node::<T>::get_state(&parent);
                match parent_state {
                    node_state => Node::<T>::rotate_up(&parent),
                    _ => Node::<T>::rotate_up(node),
                }
            }
            Node::<T>::rotate_up(node);
        }
    }
}

impl<T> binary_tree_node::Rotation for Rc<RefCell<Node<T>>>
where
    Node<T>: binary_tree_node::Update,
{
    fn rotate_left(self) -> Self {
        let sub_root = self.borrow_mut().right.take().unwrap();
        let child = sub_root.borrow_mut().left.take();
        sub_root.borrow_mut().left = Some(self.clone());
        self.borrow_mut().right = child.clone();
        Node::<T>::rotation_common_ops(&self, &sub_root, &child);
        sub_root
    }

    fn rotate_right(self) -> Self {
        let sub_root = self.borrow_mut().left.take().unwrap();
        let child = sub_root.borrow_mut().right.take();
        sub_root.borrow_mut().right = Some(self.clone());
        self.borrow_mut().left = child.clone();
        Node::<T>::rotation_common_ops(&self, &sub_root, &child);
        sub_root
    }
}

impl<T> Node<T>
where
    T: size::Size,
    Node<T>: Update,
{
    // pub fn get(
    pub fn get(node: &Rc<RefCell<Self>>, index: usize) -> Rc<RefCell<Self>> {
        assert!(index < node.borrow().size());
        let left_size = node.borrow().left.size();
        match index.cmp(&left_size) {
            std::cmp::Ordering::Less => {
                let left = node.borrow().left.as_ref().unwrap().clone();
                Self::get(&left, index)
            },
            std::cmp::Ordering::Equal => {
                Node::<T>::splay(node);
                node.clone()
            },
            std::cmp::Ordering::Greater => {
                let right = node.borrow().right.as_ref().unwrap().clone();
                Self::get(&right, index - left_size - 1)
            },
        }
    }
}

impl<T> Join for Option<Rc<RefCell<Node<T>>>>
where
    T: size::Size,
    Node<T>: Update,
{
    fn join(self, rhs: Self) -> Self {
        if self.is_none() {
            return rhs;
        }
        if rhs.is_none() {
            return self;
        }

        let left_root = Node::<T>::get(self.as_ref().unwrap(), self.size() - 1);
        rhs.as_ref().unwrap().borrow_mut().parent = Some(left_root.clone());
        left_root.borrow_mut().right = rhs;
        left_root.borrow_mut().update();
        Some(left_root)
    }
}

impl<T> Split<usize> for Option<Rc<RefCell<Node<T>>>>
where
    T: size::Size,
    Node<T>: Update,
{
    fn split(self, index: usize) -> (Self, Self) {
        let size = self.size();
        assert!(index <= size);
        if index == size {
            return (self, None);
        }
        if index == 0 {
            return (None, self);
        }
        let right_root = Node::<T>::get(self.as_ref().unwrap(), index);
        let lhs = right_root.borrow_mut().left.take();
        lhs.as_ref().unwrap().borrow_mut().parent = None;
        right_root.borrow_mut().update();
        (lhs, Some(right_root))
    }
}

impl<T: Default> Default for Node<T> {
    fn default() -> Self { Self::new(T::default()) }
}

#[derive(Debug)]
pub struct DefaultData<K, V> {
    pub size: usize,
    pub key: K,
    pub value: V,
}

impl Default for DefaultData<usize, usize> {
    fn default() -> Self { Self::new(0, 0) }
}

impl<K, V> DefaultData<K, V> {
    pub fn new(key: K, value: V) -> Self {
        DefaultData {
            size: 1,
            key,
            value,
        }
    }
}

impl<K, V> size::Size for DefaultData<K, V> {
    fn size(&self) -> usize { self.size }
}

impl<K, V> binary_tree_node::Update for Node<DefaultData<K, V>> {
    fn update(&mut self) { self.data.size = self.left.size() + self.right.size() + 1; }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::tree_node::{Insert, Pop};
        type Data = DefaultData<usize, usize>;
        type Root = Option<Rc<RefCell<Node<Data>>>>;
        let mut root = Some(Rc::new(RefCell::new(Node::new(Data::default()))));
        assert_eq!(root.size(), 1);
        root = <Root as Insert>::insert(
            root,
            0,
            Some(Rc::new(RefCell::new(Node::new(Data::new(1, 1))))),
        );
        assert_eq!(root.size(), 2);
        assert_eq!(root.as_ref().unwrap().borrow().left.size(), 0);
        assert_eq!(root.as_ref().unwrap().borrow().right.size(), 1);
        let (mut root, mut popped) = <Root as Pop>::pop(root, 0);
        assert_eq!(root.size(), 1);
        assert_eq!(popped.size(), 1);
        println!("{:?}", popped);
        println!("{:?}", root);
    }
}
