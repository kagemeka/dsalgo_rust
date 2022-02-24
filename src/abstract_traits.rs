pub trait Identity {
    fn identity() -> Self;
}
pub trait Inverse {
    fn invert(&self) -> Self;
}

pub trait Idempotent {
    const IDEMPOTENT: bool = true;
}

pub trait Commutative {
    const COMMUTATIVE: bool = true;
}

pub trait Semigroup {
    fn operate(_: &Self, _: &Self) -> Self;
}

pub trait Monoid: Semigroup + Identity {}
impl<S: Semigroup + Identity> Monoid for S {}

pub trait Group: Monoid + Inverse {}
impl<S: Monoid + Inverse> Group for S {}

pub trait AbelianGroup: Group + Commutative {}
impl<S: Group + Commutative> AbelianGroup for S {}

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
