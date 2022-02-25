use crate::abstract_traits_2;

pub trait Power<S = Self, T = abstract_traits_2::Multiplicative>:
    abstract_traits_2::Monoid<S, T>
{
    fn pow(value: &S, expontent: usize) -> S;
}

impl<S, T, M: abstract_traits_2::Monoid<S, T>> Power<S, T> for M {
    fn pow(value: &S, exponent: usize) -> S {
        if exponent == 0 {
            return M::identity();
        }
        let mut y = Self::pow(value, exponent >> 1);
        y = M::operate(&y, &y);
        if exponent & 1 == 1 {
            y = M::operate(&y, &value);
        }
        y
    }
}
