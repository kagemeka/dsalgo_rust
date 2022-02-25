use crate::abstract_traits_2::{Monoid, Multiplicative};

pub trait Power<M: Monoid<S, T>, S = M, T = Multiplicative> {
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
impl<M: Monoid<S, T>, S, T> Power<M, S, T> for M {}

// pub struct Mul;

// pub fn pow<T: abstract_traits::Monoid<Mul>>(x: &T, n: usize)
// -> T {     if n == 0 {
//         return T::identity();
//     }
//     let mut y = pow(x, n >> 1);
//     y = T::operate(&y, &y);
//     if n & 1 == 1 {
//         y = T::operate(&y, &x);
//     }
//     y
// }

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
