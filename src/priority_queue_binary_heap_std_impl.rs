use crate::priority_queue::{Pop, Push};

impl<T: std::cmp::Ord> Push for std::collections::BinaryHeap<T> {
    type T = T;

    fn push(&mut self, x: Self::T) { Self::push(self, x); }
}

impl<T: std::cmp::Ord> Pop for std::collections::BinaryHeap<T> {
    type T = T;

    fn pop(&mut self) -> Option<Self::T> { Self::pop(self) }
}
