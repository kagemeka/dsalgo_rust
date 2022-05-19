use crate::binary_operation::{BinaryOperation2, BinaryOperationId};

pub trait DistributiveProperty2<Add, Mul>:
    BinaryOperation2<Add, Lhs = Self::S, Rhs = Self::S, Codomain = Self::S>
    + BinaryOperation2<Mul, Lhs = Self::S, Rhs = Self::S, Codomain = Self::S>
where
    Add: BinaryOperationId,
    Mul: BinaryOperationId,
{
    type S;

    /// x * (y + z) = (x * y) + (x * z)
    fn assert_left_distributive(x: Self::S, y: Self::S, z: Self::S)
    where
        Self::S: Clone + PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            <Self as BinaryOperation2<Mul>>::operate(
                x.clone(),
                <Self as BinaryOperation2<Add>>::operate(y.clone(), z.clone()),
            ),
            <Self as BinaryOperation2<Add>>::operate(
                <Self as BinaryOperation2<Mul>>::operate(x.clone(), y),
                <Self as BinaryOperation2<Mul>>::operate(x, z),
            ),
        );
    }

    /// (y + z) * x = (y * x) + (z * x)
    fn assert_right_distributive(y: Self::S, z: Self::S, x: Self::S)
    where
        Self::S: Clone + PartialEq + std::fmt::Debug,
    {
        assert_eq!(
            <Self as BinaryOperation2<Mul>>::operate(
                <Self as BinaryOperation2<Add>>::operate(y.clone(), z.clone()),
                x.clone(),
            ),
            <Self as BinaryOperation2<Add>>::operate(
                <Self as BinaryOperation2<Mul>>::operate(y, x.clone()),
                <Self as BinaryOperation2<Mul>>::operate(z, x),
            ),
        );
    }

    fn assert_distributive(x: Self::S, y: Self::S, z: Self::S)
    where
        Self::S: Clone + PartialEq + std::fmt::Debug,
    {
        Self::assert_left_distributive(x.clone(), y.clone(), z.clone());
        Self::assert_right_distributive(y, z, x);
    }
}
