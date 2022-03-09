// TODO implement group theory for preset types like usize,
// min, gcd, etc.

use crate::group_theory::{
    Additive,
    Associative,
    BinaryOperation,
    Commutative,
    Identity,
    Inverse,
    Multiplicative,
};

impl<S: std::ops::Add<S, Output = S> + Copy> BinaryOperation<Additive> for S {
    fn operate(lhs: &Self, rhs: &Self) -> Self { *lhs + *rhs }
}
impl<S: std::ops::Add<S, Output = S> + Copy> Commutative<Additive> for S {}
impl<S: std::ops::Add<S, Output = S> + Copy> Associative<Additive> for S {}
impl<S: std::ops::Add<S, Output = S> + std::ops::Neg<Output = S> + Copy> Inverse<Additive> for S {
    fn invert(value: &Self) -> Self { -*value }
}

impl<S: std::ops::Mul<S, Output = S> + Copy> BinaryOperation<Multiplicative> for S {
    fn operate(lhs: &Self, rhs: &Self) -> Self { *lhs * *rhs }
}
impl<S: std::ops::Mul<S, Output = S> + Copy> Commutative<Multiplicative> for S {}
impl<S: std::ops::Mul<S, Output = S> + Copy> Associative<Multiplicative> for S {}

impl Identity<Multiplicative> for usize {
    fn identity() -> Self { 1 }
}

impl Identity<Additive> for usize {
    fn identity() -> Self { 0 }
}

impl Identity<Additive> for isize {
    fn identity() -> Self { 0 }
}

#[cfg(test)]
mod tests {
    use crate::group_theory::Multiplicative;

    #[test]
    fn test() {
        assert_eq!(<usize as crate::power::Power<Multiplicative>>::pow(&4, 2), 16,);
    }
}
