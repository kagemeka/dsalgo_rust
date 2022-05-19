pub fn is_invertible<F, X>(f: &F, x: X) -> bool
where
    F: Fn(X) -> X,
    X: Clone + PartialEq,
{
    f(f(x.clone())) == x
}
