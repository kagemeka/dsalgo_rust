pub trait BinaryOperationId {}

impl<T> BinaryOperationId for T {}

/// old implementations are mathematically wrong,
/// because if two Ids are same, comb(L, R, Codomain) are also gonna be same.
/// but it's possible that comb(L, R, Codomain) are different
/// even if Ids are same.
/// however kept.
/// because mathematically right implementations are still
/// not supported on atcoder yet (this is bug for old Rust versions).
pub trait BinaryOperation<Lhs, Rhs, Codomain, Id> {
    fn operate(lhs: Lhs, rhs: Rhs) -> Codomain;
}

/// new and mathematically right implementations.
pub trait BinaryOperation2<Id>
where
    Id: BinaryOperationId,
{
    type Lhs;
    type Rhs;
    type Codomain;
    fn operate(l: Self::Lhs, r: Self::Rhs) -> Self::Codomain;
}
