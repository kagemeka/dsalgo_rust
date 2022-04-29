use crate::group_theory;

pub trait Power<I>: group_theory::Monoid<I>
where
    I: group_theory::BinaryOperationIdentifier,
{
    // fn pow(value: &Self, expontent: usize) -> Self;
    fn pow(self, expontent: usize) -> Self;
}

impl<S, I> Power<I> for S
where
    S: group_theory::Monoid<I> + Copy,
    I: crate::group_theory::BinaryOperationIdentifier,
{
    // fn pow(value: &S, exponent: usize) -> S {
    //     if exponent == 0 {
    //         return S::identity();
    //     }
    //     let mut y = Self::pow(value, exponent >> 1);
    //     // y = S::operate(&y, &y);
    //     y = y.operate(y);
    //     if exponent & 1 == 1 {
    //         y = S::operate(&y, &value);
    //     }
    //     y
    // }

    fn pow(self, exponent: usize) -> S {
        if exponent == 0 {
            return S::identity();
        }
        let mut y = self.pow(exponent >> 1);
        // let mut y = Self::pow(value, exponent >> 1);
        // y = S::operate(&y, &y);
        y = y.operate(y);
        if exponent & 1 == 1 {
            // y = S::operate(&y, &value);
            y = y.operate(self);
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
