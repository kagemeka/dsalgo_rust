use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::Debug,
    hash::Hash,
    iter::{successors, FromIterator},
    mem::swap,
    ops::Index,
};
#[derive(Clone)]
pub struct AvlTree<T> {
    root: Option<Box<Node<T>>>,
}
impl<T> AvlTree<T> {
    pub fn new() -> Self { Self::default() }

    pub fn is_empty(&self) -> bool { self.root.is_none() }

    pub fn len(&self) -> usize { len(self.root.as_deref()) }

    pub fn push_back(&mut self, value: T) {
        self.append(&mut Self { root: Some(new(value)) })
    }

    pub fn push_front(&mut self, value: T) {
        let mut swp = Self { root: Some(new(value)) };
        swp.append(self);
        *self = swp;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let root = self.root.take()?;
        let last_index = root.len - 1;
        let (left, center, _right) = split_delete(root, last_index);
        self.root = left;
        Some(center.value)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let (_left, center, right) = split_delete(self.root.take()?, 0);
        self.root = right;
        Some(center.value)
    }

    pub fn back(&self) -> Option<&T> { self.get(self.len().checked_sub(1)?) }

    pub fn front(&self) -> Option<&T> { self.get(0) }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.get_mut(self.len().checked_sub(1)?)
    }

    pub fn front_mut(&mut self) -> Option<&mut T> { self.get_mut(0) }

    pub fn append(&mut self, other: &mut Self) {
        self.root = merge(
            self.root.take(),
            other.root.take(),
        );
    }

    pub fn split_off(&mut self, index: usize) -> Self {
        assert!(index <= self.len());
        let (left, right) = split(self.root.take(), index);
        self.root = left;
        Self { root: right }
    }

    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.len());
        let other = self.split_off(index);
        self.root = Some(merge_with_root(
            self.root.take(),
            new(value),
            other.root,
        ));
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            let (left, center, right) = split_delete(
                self.root.take().unwrap(),
                index,
            );
            self.root = merge(left, right);
            Some(center.value)
        } else {
            None
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        // let a = self.root.as_deref();
        if index < self.len() {
            Some(
                &get(
                    self.root.as_ref().unwrap(),
                    index,
                )
                .value,
            )
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.len() {
            Some(
                &mut get_mut(
                    self.root.as_mut().unwrap(),
                    index,
                )
                .value,
            )
        } else {
            None
        }
    }

    pub fn binary_search_by(
        &self,
        f: impl FnMut(&T) -> Ordering,
    ) -> Result<usize, usize> {
        binary_search_by(self.root.as_deref(), f)
    }

    pub fn binary_search_by_key<B: Ord>(
        &self,
        b: &B,
        mut f: impl FnMut(&T) -> B,
    ) -> Result<usize, usize> {
        self.binary_search_by(|x| f(x).cmp(b))
    }

    pub fn binary_search<Q: Ord>(&self, value: &Q) -> Result<usize, usize>
    where
        T: Borrow<Q>,
    {
        self.binary_search_by(|x| x.borrow().cmp(value))
    }

    pub fn partition_point(
        &self,
        mut is_right: impl FnMut(&T) -> bool,
    ) -> usize {
        partition_point(self.root.as_deref(), |node| {
            is_right(&node.value)
        })
    }

    pub fn lower_bound<Q: Ord>(&self, value: &Q) -> usize
    where
        T: Borrow<Q>,
    {
        partition_point(self.root.as_deref(), |node| {
            value <= node.value.borrow()
        })
    }

    pub fn upper_bound<Q: Ord>(&self, value: &Q) -> usize
    where
        T: Borrow<Q>,
    {
        partition_point(self.root.as_deref(), |node| {
            value < node.value.borrow()
        })
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            stack: successors(
                self.root.as_deref(),
                |current| current.left.as_deref(),
            )
            .collect(),
            rstack: successors(
                self.root.as_deref(),
                |current| current.right.as_deref(),
            )
            .collect(),
        }
    }
}
impl<T> Default for AvlTree<T> {
    fn default() -> Self { Self { root: None } }
}
impl<T: PartialEq> PartialEq for AvlTree<T> {
    fn eq(&self, other: &Self) -> bool { self.iter().eq(other) }
}
impl<T: PartialEq, A> PartialEq<[A]> for AvlTree<T>
where
    T: PartialEq<A>,
{
    fn eq(&self, other: &[A]) -> bool { self.iter().eq(other) }
}
impl<T: Eq> Eq for AvlTree<T> {}
impl<T: PartialOrd> PartialOrd for AvlTree<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}
impl<T: Ord> Ord for AvlTree<T> {
    fn cmp(&self, other: &Self) -> Ordering { self.iter().cmp(other) }
}
impl<T: Debug> Debug for AvlTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
impl<T: Hash> Hash for AvlTree<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.iter().for_each(|elm| elm.hash(state));
    }
}
impl<T> IntoIterator for AvlTree<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        let mut stack = Vec::new();
        if let Some(mut current) = self.root {
            while let Some(next) = current.left.take() {
                stack.push(current);
                current = next;
            }
            stack.push(current);
        }
        IntoIter { stack }
    }
}
impl<'a, T> IntoIterator for &'a AvlTree<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter { self.iter() }
}
impl<T> Index<usize> for AvlTree<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output { self.get(index).unwrap() }
}
impl<T> FromIterator<T> for AvlTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        fn from_slice_of_nodes<T>(
            nodes: &mut [Option<Box<Node<T>>>],
        ) -> Option<Box<Node<T>>> {
            if nodes.is_empty() {
                None
            } else {
                let i = nodes.len() / 2;
                Some(merge_with_root(
                    from_slice_of_nodes(&mut nodes[..i]),
                    nodes[i].take().unwrap(),
                    from_slice_of_nodes(&mut nodes[i + 1..]),
                ))
            }
        }
        Self {
            root: from_slice_of_nodes(
                iter.into_iter()
                    .map(new)
                    .map(Some)
                    .collect::<Vec<_>>()
                    .as_mut_slice(),
            ),
        }
    }
}
pub struct Iter<'a, T> {
    stack: Vec<&'a Node<T>>,
    rstack: Vec<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.stack.pop()?;
        self.stack.extend(successors(
            current.right.as_deref(),
            |node| node.left.as_deref(),
        ));
        if std::ptr::eq(
            current,
            *self.rstack.last().unwrap(),
        ) {
            self.stack.clear();
            self.rstack.clear();
        }
        Some(&current.value)
    }
}
impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current = self.rstack.pop()?;
        self.rstack.extend(successors(
            current.left.as_deref(),
            |node| node.right.as_deref(),
        ));
        if std::ptr::eq(
            current,
            *self.stack.last().unwrap(),
        ) {
            self.stack.clear();
            self.rstack.clear();
        }
        Some(&current.value)
    }
}
pub struct IntoIter<T> {
    stack: Vec<Box<Node<T>>>,
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = self.stack.pop()?;
        if let Some(mut current) = current.right.take() {
            while let Some(next) = current.left.take() {
                self.stack.push(current);
                current = next;
            }
            self.stack.push(current);
        }
        Some(current.value)
    }
}
#[derive(Clone)]
struct Node<T> {
    left: Option<Box<Self>>,
    right: Option<Box<Self>>,
    value: T,
    len: usize,
    ht: u8,
}
fn new<T>(value: T) -> Box<Node<T>> {
    Box::new(Node {
        left: None,
        right: None,
        value,
        len: 1,
        ht: 1,
    })
}
impl<T> Node<T> {
    fn update(&mut self) {
        self.len = len(self.left.as_deref()) + 1 + len(self.right.as_deref());
        self.ht = 1 + ht(self.left.as_deref()).max(ht(self.right.as_deref()));
    }
}
fn len<T>(tree: Option<&Node<T>>) -> usize {
    tree.as_ref().map_or(0, |node| node.len)
}
fn ht<T>(tree: Option<&Node<T>>) -> u8 {
    tree.as_ref().map_or(0, |node| node.ht)
}
fn balance<T>(node: &mut Box<Node<T>>) {
    fn rotate_left<T>(node: &mut Box<Node<T>>) {
        let mut x = node.left.take().unwrap();
        let y = x.right.take();
        swap(node, &mut x);
        x.left = y;
        x.update();
        node.right = Some(x);
        node.update();
    }
    fn rotate_right<T>(node: &mut Box<Node<T>>) {
        let mut x = node.right.take().unwrap();
        let y = x.left.take();
        swap(node, &mut x);
        x.right = y;
        x.update();
        node.left = Some(x);
        node.update();
    }
    if ht(node.left.as_deref()) > 1 + ht(node.right.as_deref()) {
        let left = node.left.as_mut().unwrap();
        if ht(left.left.as_deref()) < ht(left.right.as_deref()) {
            rotate_right(left);
        }
        rotate_left(node);
    } else if ht(node.left.as_deref()) + 1 < ht(node.right.as_deref()) {
        let right = node.right.as_mut().unwrap();
        if ht(right.left.as_deref()) > ht(right.right.as_deref()) {
            rotate_left(right);
        }
        rotate_right(node);
    } else {
        node.update();
    }
}
fn merge_with_root<T>(
    mut left: Option<Box<Node<T>>>,
    mut center: Box<Node<T>>,
    mut right: Option<Box<Node<T>>>,
) -> Box<Node<T>> {
    match ht(left.as_deref()).cmp(&ht(right.as_deref())) {
        Ordering::Less => {
            let mut root = right.take().unwrap();
            root.left = Some(merge_with_root(
                left,
                center,
                root.left.take(),
            ));
            balance(&mut root);
            root
        },
        Ordering::Equal => {
            center.left = left;
            center.right = right;
            center.update();
            center
        },
        Ordering::Greater => {
            let mut root = left.take().unwrap();
            root.right = Some(merge_with_root(
                root.right.take(),
                center,
                right,
            ));
            balance(&mut root);
            root
        },
    }
}
fn merge<T>(
    left: Option<Box<Node<T>>>,
    mut right: Option<Box<Node<T>>>,
) -> Option<Box<Node<T>>> {
    match right.take() {
        None => left,
        Some(right) => {
            let (_none, center, rhs) = split_delete(right, 0);
            Some(merge_with_root(
                left, center, rhs,
            ))
        },
    }
}
#[allow(clippy::type_complexity)]
fn split_delete<T>(
    mut root: Box<Node<T>>,
    index: usize,
) -> (
    Option<Box<Node<T>>>,
    Box<Node<T>>,
    Option<Box<Node<T>>>,
) {
    debug_assert!((0..root.len).contains(&index));
    let left = root.left.take();
    let right = root.right.take();
    let lsize = len(left.as_deref());
    match lsize.cmp(&index) {
        Ordering::Less => {
            let mut res = split_delete(
                right.unwrap(),
                index - lsize - 1,
            );
            res.0 = Some(merge_with_root(
                left, root, res.0,
            ));
            res
        },
        Ordering::Equal => (left, root, right),
        Ordering::Greater => {
            let mut res = split_delete(left.unwrap(), index);
            res.2 = Some(merge_with_root(
                res.2, root, right,
            ));
            res
        },
    }
}
#[allow(clippy::type_complexity)]
fn split<T>(
    tree: Option<Box<Node<T>>>,
    index: usize,
) -> (
    Option<Box<Node<T>>>,
    Option<Box<Node<T>>>,
) {
    match tree {
        Some(root) => {
            if root.len == index {
                (Some(root), None)
            } else {
                let (left, center, right) = split_delete(root, index);
                (
                    left,
                    Some(merge_with_root(
                        None, center, right,
                    )),
                )
            }
        },
        None => (None, None),
    }
}
fn binary_search_by<T>(
    tree: Option<&Node<T>>,
    mut f: impl FnMut(&T) -> Ordering,
) -> Result<usize, usize> {
    let node = match tree {
        None => return Err(0),
        Some(node) => node,
    };
    let lsize = len(node.left.as_deref());
    match f(&node.value) {
        Ordering::Less => binary_search_by(node.right.as_deref(), f)
            .map(|index| lsize + 1 + index)
            .map_err(|index| lsize + 1 + index),
        Ordering::Equal => Ok(lsize),
        Ordering::Greater => binary_search_by(node.left.as_deref(), f),
    }
}
fn partition_point<T>(
    tree: Option<&Node<T>>,
    mut is_right: impl FnMut(&Node<T>) -> bool,
) -> usize {
    let node = match tree {
        None => return 0,
        Some(node) => node,
    };
    let lsize = len(node.left.as_deref());
    if is_right(node) {
        partition_point(node.left.as_deref(), is_right)
    } else {
        lsize + 1 + partition_point(node.right.as_deref(), is_right)
    }
}
fn get<T>(node: &Node<T>, index: usize) -> &Node<T> {
    let lsize = len(node.left.as_deref());
    match lsize.cmp(&index) {
        Ordering::Less => get(
            node.right.as_ref().unwrap(),
            index - lsize - 1,
        ),
        Ordering::Equal => node,
        Ordering::Greater => get(
            node.left.as_ref().unwrap(),
            index,
        ),
    }
}
fn get_mut<T>(node: &mut Node<T>, index: usize) -> &mut Node<T> {
    let lsize = len(node.left.as_deref());
    match lsize.cmp(&index) {
        Ordering::Less => get_mut(
            node.right.as_mut().unwrap(),
            index - lsize - 1,
        ),
        Ordering::Equal => node,
        Ordering::Greater => get_mut(
            node.left.as_mut().unwrap(),
            index,
        ),
    }
}
