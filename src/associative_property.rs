use crate::binary_operation::BinaryOperation;

pub trait AssociativeProperty<S, Id>: BinaryOperation<S, S, S, Id> {
    fn assert_associative(first: S, second: S, third: S)
    where
        S: Copy + PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::operate(Self::operate(first, second), third),
            Self::operate(first, Self::operate(second, third)),
        );
    }
}
