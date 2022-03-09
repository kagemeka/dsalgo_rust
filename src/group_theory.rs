pub trait BinaryOperationIdentifier {}

pub trait BinaryOperation<T: BinaryOperationIdentifier> {
    fn operate(lhs: &Self, rhs: &Self) -> Self;
}

pub trait Associative<T: BinaryOperationIdentifier>: BinaryOperation<T> {}

pub trait Idempotent<T: BinaryOperationIdentifier>: BinaryOperation<T> {}

pub trait Commutative<T: BinaryOperationIdentifier>: BinaryOperation<T> {}

pub trait Identity<T: BinaryOperationIdentifier>: BinaryOperation<T> {
    fn identity() -> Self;
}

pub trait DynamicIdentity<T> {
    fn identity(&self) -> Self;
}

pub trait Inverse<T: BinaryOperationIdentifier>: BinaryOperation<T> {
    fn invert(element: &Self) -> Self;
}

pub trait Magma<T: BinaryOperationIdentifier>: BinaryOperation<T> {}
impl<T: BinaryOperationIdentifier, S: BinaryOperation<T>> Magma<T> for S {}

pub trait Semigroup<T: BinaryOperationIdentifier>: Magma<T> + Associative<T> {}
impl<T: BinaryOperationIdentifier, S: Magma<T> + Associative<T>> Semigroup<T> for S {}

pub trait Monoid<T: BinaryOperationIdentifier>: Semigroup<T> + Identity<T> {}
impl<T: BinaryOperationIdentifier, S: Semigroup<T> + Identity<T>> Monoid<T> for S {}

pub trait Group<T: BinaryOperationIdentifier>: Monoid<T> + Inverse<T> {}
impl<T: BinaryOperationIdentifier, U: Monoid<T> + Inverse<T>> Group<T> for U {}

pub trait AbelianGroup<T: BinaryOperationIdentifier>: Group<T> + Commutative<T> {}
impl<T: BinaryOperationIdentifier, S: Group<T> + Commutative<T>> AbelianGroup<T> for S {}

pub trait Semiring<T: BinaryOperationIdentifier, U: BinaryOperationIdentifier>:
    Monoid<T> + Commutative<T> + Monoid<U>
{
}
impl<T, U, S> Semiring<T, U> for S
where
    T: BinaryOperationIdentifier,
    U: BinaryOperationIdentifier,
    S: Monoid<T> + Monoid<U> + Commutative<T>,
{
}

pub trait Ring<T: BinaryOperationIdentifier, U: BinaryOperationIdentifier>:
    Semiring<T, U> + Inverse<T>
{
}
impl<T: BinaryOperationIdentifier, U: BinaryOperationIdentifier, S: Semiring<T, U> + Inverse<T>>
    Ring<T, U> for S
{
}

pub trait Default<T> {
    fn default() -> Self;
}

pub struct Additive;
impl crate::group_theory::BinaryOperationIdentifier for Additive {}
pub struct Multiplicative;
impl crate::group_theory::BinaryOperationIdentifier for Multiplicative {}

/// example of more concrete traits
pub trait AdditiveGroup: AbelianGroup<Additive> {}
impl<S: AbelianGroup<Additive>> AdditiveGroup for S {}

#[cfg(test)]
mod tests {
    fn need_semiring<S, T: super::BinaryOperationIdentifier, U: super::BinaryOperationIdentifier>()
    where
        S: super::Semiring<T, U> + std::fmt::Debug + PartialEq,
    {
        let add_e = <S as super::Identity<T>>::identity();
        let value_add = <S as super::BinaryOperation<T>>::operate(&add_e, &add_e);
        assert_eq!(value_add, add_e);

        let mul_e = <S as super::Identity<U>>::identity();
        let value_mul = <S as super::BinaryOperation<U>>::operate(&mul_e, &mul_e);
        assert_eq!(value_mul, mul_e);
        eprintln!("{:?}", value_add);
    }

    #[test]
    fn test() { need_semiring::<usize, super::Additive, super::Multiplicative>(); }
}
