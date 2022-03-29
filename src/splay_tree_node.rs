use std::{cell::RefCell, rc::Rc};
pub struct Node<T> {
    pub data: T,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
    pub parent: Option<Rc<RefCell<Self>>>,
    // pub right: Option<Box<Self>>,
    // pub parent: Option<Box<Self>>,
}

use crate::{binary_tree_node, binary_tree_node::Update, size, size::Size};
impl<T: size::Size> size::Size for Node<T> {
    fn size(&self) -> usize { self.data.size() }
}

impl<T: size::Size> size::Size for Option<Node<T>> {
    fn size(&self) -> usize {
        match self {
            Some(node) => node.data.size(),
            None => 0,
        }
    }
}
impl<T: size::Size> size::Size for Option<Rc<RefCell<Node<T>>>> {
    fn size(&self) -> usize {
        match self {
            Some(node) => node.borrow().data.size(),
            None => 0,
        }
    }
}

#[derive(PartialEq)]
enum State {
    LeftChild,
    RightChild,
    Root,
}

// type Cell<T> = Rc<RefCell<Node<T>>>;
use crate::binary_tree_node::Rotation;
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
        let some_parent = &node.borrow().parent;
        let parent = some_parent.as_ref().unwrap();
        if parent.borrow().left.is_some()
            && Rc::ptr_eq(parent.borrow().left.as_ref().unwrap(), node)
        {
            parent.clone().rotate_right();
        } else {
            parent.clone().rotate_left();
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
        while let Some(parent) = &node.borrow().parent {
            let node_state = Node::<T>::get_state(node);
            if parent.borrow().parent.is_some() {
                let parent_state = Node::<T>::get_state(parent);
                if parent_state == node_state {
                    Node::<T>::rotate_up(parent);
                } else {
                    Node::<T>::rotate_up(node);
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
        self.borrow_mut().right = child;
        Node::<T>::rotation_common_ops(&self, &sub_root, &self.borrow_mut().right);
        sub_root
    }

    fn rotate_right(self) -> Self {
        let sub_root = self.borrow_mut().left.take().unwrap();
        let child = sub_root.borrow_mut().right.take();
        sub_root.borrow_mut().right = Some(self.clone());
        self.borrow_mut().left = child;
        Node::<T>::rotation_common_ops(&self, &sub_root, &self.borrow_mut().left);
        sub_root
    }
}

pub struct DefaultData<K, V> {
    pub size: usize,
    pub key: K,
    pub value: V,
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
    fn update(&mut self) { self.data.size = self.left.size(); }
}
