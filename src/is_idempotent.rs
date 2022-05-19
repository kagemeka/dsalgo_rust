pub fn is_idempotent<F, X>(f: &F, x: X) -> bool
where
    F: Fn(X, X) -> X,
    X: Clone + PartialEq,
{
    f(x.clone(), x.clone()) == x
}
