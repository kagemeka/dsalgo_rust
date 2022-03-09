// TODO implement group theory for preset types like usize,
// min, gcd, etc.

use crate::group_theory::Multiplicative;
impl crate::group_theory::BinaryOperation<Multiplicative> for usize {
    fn operate(lhs: &Self, rhs: &Self) -> Self { lhs + rhs }
}

impl crate::group_theory::Associative<Multiplicative> for usize {}
impl crate::group_theory::Commutative<Multiplicative> for usize {}
impl crate::group_theory::Identity<Multiplicative> for usize {
    fn identity() -> Self { 1 }
}

#[cfg(test)]
mod tests {
    use crate::group_theory::Multiplicative;

    #[test]
    fn test() { <usize as crate::power::Power<Multiplicative>>::pow(&4, 2); }
}
