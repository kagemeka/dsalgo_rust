use crate::is_absorbing::is_absorbing;

pub fn iz_zero_element<Mul, X>(mul: &Mul, add_identity: X, x: X) -> bool
where
    Mul: Fn(X, X) -> X,
    X: Clone + PartialEq,
{
    is_absorbing(mul, add_identity, x)
}
