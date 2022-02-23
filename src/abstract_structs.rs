/// Fn(&S, &S) -> S is a trait.
/// this is a dynamic size object at compilation time.
/// thus, it's needed to be enclosed with Box<dyn>
/// pointer.
pub struct Monoid<'a, S> {
    pub op: &'a dyn Fn(&S, &S) -> S,
    pub e: &'a dyn Fn() -> S,
    pub commutative: bool,
    pub idempotent: bool,
}

pub struct Semigroup<'a, S> {
    pub op: &'a dyn Fn(&S, &S) -> S,
    pub commutative: bool,
    pub idempotent: bool,
}
