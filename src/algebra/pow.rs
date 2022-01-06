use crate::algebra::abstract_::structure::{traits::Monoid, structs};

pub fn pow<T: Monoid>(x: &T, n: usize) -> T {
    if n == 0 { return T::e(); }
    let mut y = pow(x, n >> 1);
    y = T::op(&y, &y);
    if n & 1 == 1 { y = T::op(&y, &x); }
    y
}


pub struct Power<'a, T> {
    m: structs::Monoid<'a, T>,
}

impl<'a, T> Power<'a, T> {
    pub fn new(m: structs::Monoid<'a, T>) -> Self { Self { m } }

    pub fn r#do(&self, x: &T, n: usize) -> T{
        if n == 0 { return (self.m.e)(); }
        let mut y = self.r#do(x, n >> 1);
        y = (self.m.op)(&y, &y);
        if n & 1 == 1 { y = (self.m.op)(&y, &x); }
        y
    }
}