pub fn is_left_absorbing<F, X>(f: &F, element: X, x: X) -> bool
where
    F: Fn(X, X) -> X,
    X: Clone + PartialEq,
{
    f(element.clone(), x) == element
}

pub fn is_right_absorbing<F, X>(f: &F, element: X, x: X) -> bool
where
    F: Fn(X, X) -> X,
    X: Clone + PartialEq,
{
    f(x, element.clone()) == element
}

pub fn is_absorbing<F, X>(f: &F, element: X, x: X) -> bool
where
    F: Fn(X, X) -> X,
    X: Clone + PartialEq,
{
    is_left_absorbing(f, element.clone(), x.clone())
        && is_right_absorbing(f, element, x)
}
