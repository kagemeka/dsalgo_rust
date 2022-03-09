pub struct Shape {
    pub height: usize,
    pub width: usize,
}

pub trait Matrix: std::ops::IndexMut<(usize, usize)> {}

struct Add;
impl crate::group_theory::BinaryOperationIdentifier for Add {}

// impl<M: Matrix> crate::group_theory::Identity<Add> for
// Option<M> {     fn identity() -> Self { None }
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
