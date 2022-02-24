use crate::{abstract_structs, abstract_traits};

pub fn pow<T: abstract_traits::Monoid>(x: &T, n: usize) -> T {
    if n == 0 {
        return T::identity();
    }
    let mut y = pow(x, n >> 1);
    y = T::operate(&y, &y);
    if n & 1 == 1 {
        y = T::operate(&y, &x);
    }
    y
}

pub struct Power<'a, T> {
    monoid: abstract_structs::Monoid<'a, T>,
}

impl<'a, T> Power<'a, T> {
    pub fn new(monoid: abstract_structs::Monoid<'a, T>) -> Self {
        Self { monoid }
    }

    pub fn r#do(&self, x: &T, n: usize) -> T {
        if n == 0 {
            return (self.monoid.identity)();
        }
        let mut y = self.r#do(x, n >> 1);
        y = (self.monoid.operate)(&y, &y);
        if n & 1 == 1 {
            y = (self.monoid.operate)(&y, &x);
        }
        y
    }
}
