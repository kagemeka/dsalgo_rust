use crate::binary_operation::{BinaryOperation2, BinaryOperationId};

pub trait LeftAbsorbingElement<Id>:
    BinaryOperation2<Id, Lhs = Self::X, Rhs = Self::X, Codomain = Self::X>
where
    Id: BinaryOperationId,
{
    type X;

    fn left_absorbing_element() -> Self::X;

    fn assert_left_absorbing(element: Self::X)
    where
        Self::X: PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::operate(
                Self::left_absorbing_element(),
                element,
            ),
            Self::left_absorbing_element(),
        );
    }
}

pub trait RightAbsorbingElement<Id>:
    BinaryOperation2<Id, Lhs = Self::X, Rhs = Self::X, Codomain = Self::X>
where
    Id: BinaryOperationId,
{
    type X;

    fn right_absorbing_element() -> Self::X;

    fn assert_right_absorbing(element: Self::X)
    where
        Self::X: PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::operate(
                element,
                Self::right_absorbing_element(),
            ),
            Self::right_absorbing_element(),
        );
    }
}

pub trait AbsorbingElement<Id>:
    LeftAbsorbingElement<Id, X = <Self as AbsorbingElement<Id>>::X>
    + RightAbsorbingElement<Id, X = <Self as AbsorbingElement<Id>>::X>
where
    Id: BinaryOperationId,
{
    type X;
    fn absorbing_element() -> <Self as AbsorbingElement<Id>>::X {
        Self::left_absorbing_element()
    }

    fn assert_same_left_right()
    where
        <Self as AbsorbingElement<Id>>::X: PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            Self::left_absorbing_element(),
            Self::right_absorbing_element()
        );
    }

    fn assert_absorbing(element: <Self as AbsorbingElement<Id>>::X)
    where
        <Self as AbsorbingElement<Id>>::X: Clone + PartialEq + std::fmt::Debug,
    {
        Self::assert_same_left_right();
        Self::assert_left_absorbing(element.clone());
        Self::assert_right_absorbing(element);
    }
}
