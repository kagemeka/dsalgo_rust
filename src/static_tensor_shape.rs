//! static tensor shape for static tensor.

use crate::tensor_property::Shape;

/// example of static tensor shape.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TensorShape2_3_4;
impl Shape for TensorShape2_3_4 {
    fn shape() -> &'static [usize] { &[2, 3, 4] }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tensor::Tensor;
    #[test]
    fn test() {
        type Ten = Tensor<TensorShape2_3_4, i64>;

        let a = Ten::new();
        println!("{:?}", a);
    }
}
