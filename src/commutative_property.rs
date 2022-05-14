use crate::binary_operation::BinaryOperation;

pub trait CommutativeProperty<S, T, Id>: BinaryOperation<S, S, T, Id> {
    fn assert_commutative(a: S, b: S)
    where
        S: Clone,
        T: PartialEq + std::fmt::Debug,
    {
        assert_eq!(Self::operate(a.clone(), b.clone()), Self::operate(b, a));
    }
}
