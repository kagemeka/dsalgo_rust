use crate::binary_operation::BinaryOperation;

pub trait Idempotence<S, Id>: BinaryOperation<S, S, S, Id> {
    fn assert_idempotent(element: S)
    where
        S: Clone + PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::operate(
                element.clone(),
                element.clone()
            ),
            element
        );
    }
}
