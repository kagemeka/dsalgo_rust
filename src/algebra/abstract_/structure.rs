pub mod structs {
    /// Fn(&S, &S) -> S is a trait.
    /// this is a dynamic size object at compilation time.
    /// thus, it's needed to be enclosed with Box<dyn> pointer.
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
}

pub mod traits {
    pub trait Identity {
        fn e() -> Self;
    }
    pub trait Inverse {
        fn inv(&self) -> Self;
    }
    pub trait Semigroup {
        fn op(_: &Self, _: &Self) -> Self;
        const COMMUTATIVE: bool;
        const IDEMPOTENT: bool;
    }

    pub trait Monoid: Semigroup + Identity {}
    pub trait Group: Monoid + Inverse {}
    pub trait MulIdentity {
        fn e() -> Self;
    }

    pub trait AddIdentity {
        fn e() -> Self;
    }
    pub trait AddInverse {
        fn inv(&self) -> Self;
    }
    pub trait MulInverse {
        fn inv(&self) -> Self;
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
}
