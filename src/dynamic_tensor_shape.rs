//! dynamic tensor shape for static tensor.

use std::lazy::SyncOnceCell;

use crate::tensor_property::Shape;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynamicTensorShape<Id>(std::marker::PhantomData<Id>);

impl<Id> DynamicTensorShape<Id> {
    fn cell() -> &'static SyncOnceCell<Vec<usize>> {
        static CELL: SyncOnceCell<Vec<usize>> = SyncOnceCell::new();
        &CELL
    }

    /// use empty (0-dim) slice [] for scalar.
    pub fn set(shape: &[usize]) { Self::cell().set(shape.to_vec()).unwrap(); }
}

impl<Id> Shape for DynamicTensorShape<Id> {
    fn shape() -> &'static [usize] { Self::cell().get().unwrap() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tensor::Tensor;
    #[test]
    fn test() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct PseudoId;

        type S = DynamicTensorShape<PseudoId>;
        S::set(&[2, 3, 4]);
        type Ten = Tensor<S, i64>;

        let a = Ten::new();
        println!("{:?}", a);
    }
}
