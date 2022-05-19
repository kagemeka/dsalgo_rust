pub fn is_commutative<F, X, Y>(f: &F, a: X, b: X) -> bool
where
    F: Fn(X, X) -> Y,
    X: Clone,
    Y: PartialEq,
{
    f(a.clone(), b.clone()) == f(b, a)
}
