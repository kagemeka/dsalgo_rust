pub fn is_left_distributive<Add, Mul, X>(
    add: &Add,
    mul: &Mul,
    x: X,
    y: X,
    z: X,
) -> bool
where
    Add: Fn(X, X) -> X,
    Mul: Fn(X, X) -> X,
    X: Clone + PartialEq,
{
    mul(
        x.clone(),
        add(y.clone(), z.clone()),
    ) == add(mul(x.clone(), y), mul(x, z))
}

pub fn is_right_distributive<Add, Mul, X>(
    add: &Add,
    mul: &Mul,
    y: X,
    z: X,
    x: X,
) -> bool
where
    Add: Fn(X, X) -> X,
    Mul: Fn(X, X) -> X,
    X: Clone + PartialEq,
{
    mul(
        add(y.clone(), z.clone()),
        x.clone(),
    ) == add(mul(y, x.clone()), mul(z, x))
}

pub fn is_distributive<Add, Mul, X>(
    add: &Add,
    mul: &Mul,
    x: X,
    y: X,
    z: X,
) -> bool
where
    Add: Fn(X, X) -> X,
    Mul: Fn(X, X) -> X,
    X: Clone + PartialEq,
{
    is_left_distributive(
        add,
        mul,
        x.clone(),
        y.clone(),
        z.clone(),
    ) && is_right_distributive(add, mul, y, z, x)
}
