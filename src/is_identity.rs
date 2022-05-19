pub fn is_left_identity<F, S>(f: &F, identity: S, x: S) -> bool
where
    F: Fn(S, S) -> S,
    S: Clone + PartialEq,
{
    f(identity, x.clone()) == x
}

pub fn is_right_identity<F, S>(f: &F, identity: S, x: S) -> bool
where
    F: Fn(S, S) -> S,
    S: Clone + PartialEq,
{
    f(x.clone(), identity) == x
}

pub fn is_identity<F, S>(f: &F, identity: S, x: S) -> bool
where
    F: Fn(S, S) -> S,
    S: Clone + PartialEq,
{
    is_left_identity(f, identity.clone(), x.clone())
        && is_right_identity(f, identity, x)
}
