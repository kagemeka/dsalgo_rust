pub fn factorial<T>(n: usize) -> T
where
    T: std::ops::Mul<Output = T> + From<usize>,
{
    (1..=n).fold(1.into(), |accum, x| {
        accum * x.into()
    })
}
