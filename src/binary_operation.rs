pub trait BinaryOperationId {}

impl<T> BinaryOperationId for T {}

pub trait BinaryOperation<Id: BinaryOperationId> {
    type Lhs;
    type Rhs;
    type Codomain;
    fn map(l: Self::Lhs, r: Self::Rhs) -> Self::Codomain;
}
