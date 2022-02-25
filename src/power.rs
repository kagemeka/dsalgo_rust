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

// pub struct Power<'a, T> {
//     monoid: abstract_structs::Monoid<'a, T>,
// }

// impl<'a, T> Power<'a, T> {
//     pub fn new(monoid: abstract_structs::Monoid<'a, T>) ->
// Self {         Self { monoid }
//     }

//     pub fn r#do(&self, x: &T, n: usize) -> T {
//         if n == 0 {
//             return (self.monoid.identity)();
//         }
//         let mut y = self.r#do(x, n >> 1);
//         y = (self.monoid.operate)(&y, &y);
//         if n & 1 == 1 {
//             y = (self.monoid.operate)(&y, &x);
//         }
//         y
//     }
// }
