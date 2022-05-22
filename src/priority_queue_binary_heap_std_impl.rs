use crate::priority_queue::{MinimumQueue, Pop, Push};

impl<T: std::cmp::Ord> Push
    for std::collections::BinaryHeap<std::cmp::Reverse<T>>
{
    type T = T;

    fn push(&mut self, x: Self::T) { Self::push(self, std::cmp::Reverse(x)); }
}

impl<T: std::cmp::Ord> Pop
    for std::collections::BinaryHeap<std::cmp::Reverse<T>>
{
    type T = T;

    fn pop(&mut self) -> Option<Self::T> {
        if let Some(std::cmp::Reverse(x)) = Self::pop(self) {
            Some(x)
        } else {
            None
        }
    }
}

impl<T: std::cmp::Ord> MinimumQueue
    for std::collections::BinaryHeap<std::cmp::Reverse<T>>
{
}
