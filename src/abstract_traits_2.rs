pub trait Identity<S, T> {
    fn identity() -> S;
}
pub trait Inverse<S, T> {
    fn invert(_: &S) -> S;
}

pub trait Idempotent<S, T> {}

pub trait Commutative<S, T> {}

pub trait Semigroup<S, T> {
    fn operate(_: &S, _: &S) -> S;
}

pub trait Monoid<S, T>: Semigroup<S, T> + Identity<S, T> {}
impl<S, T, U: Semigroup<S, T> + Identity<S, T>> Monoid<S, T> for U {}

pub trait Group<S, T>: Monoid<S, T> + Inverse<S, T> {}
impl<S, T, U: Monoid<S, T> + Inverse<S, T>> Group<S, T> for U {}

pub trait AbelianGroup<S, T>: Group<S, T> + Commutative<S, T> {}
impl<S, T, U: Group<S, T> + Commutative<S, T>> AbelianGroup<S, T> for U {}

pub trait Semiring<S, Add, Mul>:
    Monoid<S, Add> + Monoid<S, Mul> + Commutative<S, Add>
{
}
impl<S, Add, Mul, U> Semiring<S, Add, Mul> for U where
    U: Monoid<S, Add> + Monoid<S, Mul> + Commutative<S, Add>
{
}

pub trait Ring<S, Add, Mul>: Semiring<S, Add, Mul> + Inverse<S, Add> {}
impl<S, Add, Mul, U> Ring<S, Add, Mul> for U where
    U: Semiring<S, Add, Mul> + Inverse<S, Add>
{
}

pub trait Default<S, T> {
    fn default() -> S;
}

pub struct Additive;
pub struct Multiplicative;

#[cfg(test)]
mod tests {
    // struct ExampleSemiring<S>(std::marker::PhantomData<S>);
    struct UsizeAddMul;

    impl super::Identity<usize, super::Additive> for UsizeAddMul {
        fn identity() -> usize { 0 }
    }

    impl super::Identity<usize, super::Multiplicative> for UsizeAddMul {
        fn identity() -> usize { 1 }
    }

    impl super::Semigroup<usize, super::Additive> for UsizeAddMul {
        fn operate(a: &usize, b: &usize) -> usize { a + b }
    }

    impl super::Semigroup<usize, super::Multiplicative> for UsizeAddMul {
        fn operate(a: &usize, b: &usize) -> usize { a * b }
    }

    impl super::Commutative<usize, super::Additive> for UsizeAddMul {}

    fn need_semiring<S, Add, Mul, U>()
    where
        U: super::Semiring<S, Add, Mul>,
        S: std::fmt::Debug + PartialEq,
    {
        let add_e = <U as super::Identity<S, Add>>::identity();
        let value_add =
            <U as super::Semigroup<S, Add>>::operate(&add_e, &add_e);
        assert_eq!(value_add, add_e);

        let mul_e = <U as super::Identity<S, Mul>>::identity();
        let value_mul =
            <U as super::Semigroup<S, Mul>>::operate(&mul_e, &mul_e);
        assert_eq!(value_mul, mul_e);
        eprintln!("{:?}", value_add);
    }

    #[test]
    fn test() {
        need_semiring::<
            usize,
            super::Additive,
            super::Multiplicative,
            UsizeAddMul,
        >();
    }
}
