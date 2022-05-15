// TODO implement group theory for preset types like usize,
// min, gcd, etc.

use crate::group_theory::{
    Additive,
    AssociativeProperty,
    BinaryOperation,
    CommutativeProperty,
    IdentityElement,
    InverseElement,
    Multiplicative,
};

impl<S: std::ops::Add<S, Output = S>> BinaryOperation<Self, Self, Additive>
    for S
{
    fn operate(self, rhs: Self) -> Self { self + rhs }
}
impl<S: std::ops::Add<S, Output = S>> CommutativeProperty<Self, Additive>
    for S
{
}
impl<S: std::ops::Add<S, Output = S>> AssociativeProperty<Additive> for S {}
impl<
    S: std::ops::Add<S, Output = S>
        + std::ops::Neg<Output = S>
        + IdentityElement<Additive>,
> InverseElement<Additive> for S
{
    // fn invert(value: &Self) -> Self { -*value }

    fn invert(self) -> Self { -self }
}

impl<S: std::ops::Mul<S, Output = S>>
    BinaryOperation<Self, Self, Multiplicative> for S
{
    fn operate(self, rhs: Self) -> Self { self * rhs }
}
impl<S: std::ops::Mul<S, Output = S>> CommutativeProperty<Self, Multiplicative>
    for S
{
}
impl<S: std::ops::Mul<S, Output = S>> AssociativeProperty<Multiplicative>
    for S
{
}

impl IdentityElement<Multiplicative> for usize {
    fn identity() -> Self { 1 }
}

impl IdentityElement<Additive> for usize {
    fn identity() -> Self { 0 }
}

impl IdentityElement<Additive> for isize {
    fn identity() -> Self { 0 }
}

impl IdentityElement<Additive> for i32 {
    fn identity() -> Self { 0 }
}

#[cfg(test)]
mod tests {
    use crate::group_theory::Multiplicative;

    #[test]
    fn test() {
        // assert_eq!(<usize as
        // crate::power::Power<Multiplicative>>::pow(&4, 2), 16,);
        assert_eq!(
            <usize as crate::power::Power<Multiplicative>>::pow(4, 2),
            16
        );
    }
}
