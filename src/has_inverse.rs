pub fn has_inverse<F, X>(f: &F, x: X) -> bool
where
    F: Fn(X) -> X,
    X: Clone + PartialEq,
{
    f(f(x.clone())) == x
}
