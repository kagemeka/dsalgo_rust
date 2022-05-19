use crate::{
    binary_operation::BinaryOperationId,
    inverse_element::InverseElement,
    semiring::Semiring,
};

pub trait Ring<Add, Mul>: Semiring<Add, Mul>
where
    Add: BinaryOperationId,
    Mul: BinaryOperationId,
{
    fn add_inv(element: Self::S) -> Self::S;
}

impl<S, Add, Mul, T> Ring<Add, Mul> for T
where
    T: Semiring<Add, Mul, S = S> + InverseElement<Add, X = S>,
    Add: BinaryOperationId,
    Mul: BinaryOperationId,
{
    fn add_inv(element: Self::S) -> Self::S {
        <T as InverseElement<Add>>::invert(element)
    }
}
