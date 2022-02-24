pub trait Infinity {
    const INFINITY: Self;
}

impl Infinity for usize {
    const INFINITY: usize = std::usize::MAX;
}
