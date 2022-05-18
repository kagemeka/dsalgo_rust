use crate::{
    absorbing_element::AbsorbingElement,
    binary_operation::BinaryOperationId,
    identity_element::IdentityElement2,
};

pub trait ZeroElement<Add, Mul>:
    IdentityElement2<Add>
    + AbsorbingElement<Mul, X = <Self as IdentityElement2<Add>>::X>
where
    Add: BinaryOperationId,
    Mul: BinaryOperationId,
{
    // type X;
    fn zero() -> <Self as IdentityElement2<Add>>::X { Self::identity() }

    fn assert_zero(element: <Self as IdentityElement2<Add>>::X)
    where
        <Self as IdentityElement2<Add>>::X: PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::identity(),
            Self::absorbing_element()
        );
    }
}
