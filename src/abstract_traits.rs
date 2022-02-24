pub trait Identity {
    fn identity() -> Self;
}
pub trait Inverse {
    fn invert(&self) -> Self;
}
pub trait Semigroup {
    fn operate(_: &Self, _: &Self) -> Self;
    const COMMUTATIVE: bool;
    const IDEMPOTENT: bool;
}

pub trait Monoid: Semigroup + Identity {}
pub trait Group: Monoid + Inverse {}
pub trait MulIdentity {
    fn identity() -> Self;
}

pub trait AddIdentity {
    fn identity() -> Self;
}
pub trait AddInverse {
    fn invert(&self) -> Self;
}
pub trait MulInverse {
    fn invert(&self) -> Self;
}
pub trait Semiring:
    Sized
    + std::ops::Add<Output = Self>
    + std::ops::Mul<Output = Self>
    + AddIdentity
    + MulIdentity
{
    const MUL_COMMUTATIVE: bool;
    const ADD_IDEMPOTNET: bool;
}
pub trait Ring: Semiring + AddInverse {}
