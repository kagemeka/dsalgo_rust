pub fn factorial<T>(n: u64) -> T
where
    T: std::ops::Mul<Output = T> + From<u64>,
{
    (1..=n).fold(1.into(), |accum, x| {
        accum * x.into()
    })
}
