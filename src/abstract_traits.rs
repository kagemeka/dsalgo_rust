pub trait Identity<T> {
    fn identity() -> Self;
}
pub trait Inverse<T> {
    fn invert(_: &Self) -> Self;
}

pub trait Idempotent<T> {
    const IDEMPOTENT: bool = true;
}

pub trait Commutative<T> {
    const COMMUTATIVE: bool = true;
}

pub trait Semigroup<T> {
    fn operate(_: &Self, _: &Self) -> Self;
}

pub trait Monoid<T>: Semigroup<T> + Identity<T> {}
impl<T, S: Semigroup<T> + Identity<T>> Monoid<T> for S {}

pub trait Group<T>: Monoid<T> + Inverse<T> {}
impl<T, S: Monoid<T> + Inverse<T>> Group<T> for S {}

pub trait AbelianGroup<T>: Group<T> + Commutative<T> {}
impl<T, S: Group<T> + Commutative<T>> AbelianGroup<T> for S {}

pub trait Semiring<Add, Mul>:
    Monoid<Add> + Monoid<Mul> + Commutative<Add>
{
}
impl<Add, Mul, S> Semiring<Add, Mul> for S where
    S: Monoid<Add> + Monoid<Mul> + Commutative<Add>
{
}

pub trait Ring<Add, Mul>: Semiring<Add, Mul> + Inverse<Add> {}
impl<Add, Mul, S: Semiring<Add, Mul> + Inverse<Add>> Ring<Add, Mul> for S {}

pub trait Default<T> {
    fn default() -> Self;
}
