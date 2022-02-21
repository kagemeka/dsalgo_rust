pub trait Inf {
    const INF: Self;
}

impl Inf for usize {
    const INF: usize = std::usize::MAX;
}
