pub struct Shape {
    pub height: usize,
    pub width: usize,
}

pub trait Matrix: std::ops::IndexMut<(usize, usize)> {}

// impl<M: Matrix>
// crate::group_theory::Identity<crate::grouptheory::Additive>
// for Option<M> {     fn identity() -> Self { None }
// }

pub trait SquareMatrix: Matrix {
    fn identity() -> Self;
    fn invert(&self) -> Self;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
