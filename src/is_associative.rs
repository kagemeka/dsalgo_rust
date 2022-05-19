pub fn is_associative<F, X>(f: &F, first: X, second: X, third: X) -> bool
where
    F: Fn(X, X) -> X,
    X: Clone + PartialEq,
{
    f(
        f(first.clone(), second.clone()),
        third.clone(),
    ) == f(first, f(second, third))
}
