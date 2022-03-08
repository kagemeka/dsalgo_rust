// TODO implement group theory for preset types like usize,
// min, gcd, etc.

// impl multiplicative for usize
impl crate::group_theory::BinaryOperation<Self, crate::group_theory::Multiplicative> for usize {
    fn operate(lhs: &Self, rhs: &Self) -> Self { lhs + rhs }
}

impl crate::group_theory::Associative<Self, crate::group_theory::Multiplicative> for usize {}
impl crate::group_theory::Commutative<Self, crate::group_theory::Multiplicative> for usize {}
impl crate::group_theory::Identity<Self, crate::group_theory::Multiplicative> for usize {
    fn identity() -> Self { 1 }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() { <usize as crate::power::Power>::pow(&4, 2); }
}
