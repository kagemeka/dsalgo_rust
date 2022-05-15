pub trait BinaryOperationIdentifier {}

/// F x Self -> T
/// mathematically, it's enough to clarify F and I
/// because if Self, F, I are known, T is also gonna be known.
/// but compiler cannot know that.
pub trait BinaryOperation<F: ?Sized, T: ?Sized, I: BinaryOperationIdentifier> {
    fn operate(self, operator: F) -> T;
}

/// first * second * third
pub trait AssociativeProperty<I: BinaryOperationIdentifier>:
    BinaryOperation<Self, Self, I>
{
    fn assert_associative(first: Self, second: Self, third: Self)
    where
        Self: Copy + PartialEq + Debug,
    {
        assert_eq!(
            first.operate(second).operate(third),
            first.operate(second.operate(third))
        );
    }
}

pub trait Idempotence<I: BinaryOperationIdentifier>:
    BinaryOperation<Self, Self, I>
{
    fn assert_idempotent(self)
    where
        Self: Copy + PartialEq + Debug,
    {
        assert_eq!(self.operate(self), self);
    }
}

pub trait CommutativeProperty<T: Sized, I: BinaryOperationIdentifier>:
    BinaryOperation<Self, T, I> + Sized
{
    fn assert_commutative(self, operator: Self)
    where
        Self: Copy,
        T: PartialEq + Debug,
    {
        assert_eq!(
            self.operate(operator),
            operator.operate(self)
        );
    }
}

pub trait IdentityElement<I: BinaryOperationIdentifier>:
    BinaryOperation<Self, Self, I>
{
    fn identity() -> Self;
}

// TODO: make more sophisticated.
// pub trait DynamicIdentity<T> {
//     fn identity(&self) -> Self;
// }

// pub trait Inverse<T: BinaryOperationIdentifier>:
// BinaryOperation<T> {     fn invert(element: &Self) -> Self;
// }

use std::fmt::Debug;

pub trait InverseElement<I: BinaryOperationIdentifier>:
    IdentityElement<I>
{
    fn invert(self) -> Self;
}

pub trait Magma<I: BinaryOperationIdentifier>:
    BinaryOperation<Self, Self, I>
{
}
impl<I: BinaryOperationIdentifier, S: BinaryOperation<Self, Self, I>> Magma<I>
    for S
{
}

pub trait Semigroup<I: BinaryOperationIdentifier>:
    Magma<I> + AssociativeProperty<I>
{
}
impl<I: BinaryOperationIdentifier, S: Magma<I> + AssociativeProperty<I>>
    Semigroup<I> for S
{
}

pub trait Monoid<I: BinaryOperationIdentifier>:
    Semigroup<I> + IdentityElement<I>
{
}
impl<I: BinaryOperationIdentifier, S: Semigroup<I> + IdentityElement<I>>
    Monoid<I> for S
{
}

pub trait CommutativeMonoid<I: BinaryOperationIdentifier>:
    Monoid<I> + CommutativeProperty<Self, I> + Sized
{
}
impl<I, S> CommutativeMonoid<I> for S
where
    I: BinaryOperationIdentifier,
    S: Monoid<I> + CommutativeProperty<Self, I>,
{
}

pub trait Group<I: BinaryOperationIdentifier>:
    Monoid<I> + InverseElement<I>
{
}
impl<I: BinaryOperationIdentifier, S: Monoid<I> + InverseElement<I>> Group<I>
    for S
{
}

pub trait AbelianGroup<I: BinaryOperationIdentifier>:
    Group<I> + CommutativeProperty<Self, I>
{
}
impl<I: BinaryOperationIdentifier, S: Group<I> + CommutativeProperty<Self, I>>
    AbelianGroup<I> for S
{
}

pub trait Semiring<I, J>: CommutativeMonoid<I> + Monoid<J>
where
    I: BinaryOperationIdentifier,
    J: BinaryOperationIdentifier,
{
}
impl<I, J, S> Semiring<I, J> for S
where
    I: BinaryOperationIdentifier,
    J: BinaryOperationIdentifier,
    S: CommutativeMonoid<I> + Monoid<J>,
{
}

pub trait Ring<I, J>: Semiring<I, J> + InverseElement<I>
where
    I: BinaryOperationIdentifier,
    J: BinaryOperationIdentifier,
{
}
impl<I, J, S: Semiring<I, J> + InverseElement<I>> Ring<I, J> for S
where
    I: BinaryOperationIdentifier,
    J: BinaryOperationIdentifier,
{
}

pub trait Default<I: BinaryOperationIdentifier>:
    BinaryOperation<Self, Self, I> + Sized
{
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
    fn need_semiring<
        S,
        I: super::BinaryOperationIdentifier,
        J: super::BinaryOperationIdentifier,
    >()
    where
        S: super::Semiring<I, J> + std::fmt::Debug + PartialEq + Copy,
    {
        let add_e = <S as super::IdentityElement<I>>::identity();
        // let value_add = add_e.operate(add_e);
        let value_add =
            <S as super::BinaryOperation<_, _, I>>::operate(add_e, add_e);
        // let value_add = <S as
        // super::BinaryOperation<T>>::operate(&add_e, &add_e);
        assert_eq!(value_add, add_e);

        let mul_e = <S as super::IdentityElement<J>>::identity();
        // let value_mul: S = mul_e.operate(mul_e);
        let value_mul =
            <S as super::BinaryOperation<_, _, J>>::operate(mul_e, mul_e);
        // super::BinaryOperation<U>>::operate(&mul_e, &mul_e);
        assert_eq!(value_mul, mul_e);
        eprintln!("{:?}", value_add);
    }

    #[test]
    fn test() {
        need_semiring::<usize, super::Additive, super::Multiplicative>();
    }
}
