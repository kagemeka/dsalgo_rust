#[allow(dead_code)]
pub type DijkstraQueueBinaryHeapStd =
    std::collections::BinaryHeap<std::cmp::Reverse<(u64, usize)>>;
