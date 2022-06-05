use crate::accumulate::accumulate;

pub fn factorial_table<T>(size: usize) -> Vec<T>
where
    T: std::ops::Mul<Output = T> + From<u64> + Clone,
{
    if size == 0 {
        return vec![];
    }
    let mut v = (0..size as u64).map(|i| i.into()).collect::<Vec<T>>();
    v[0] = 1.into();
    let op = |a: T, b: T| -> T { a * b };
    accumulate(&op, v.into_iter()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        use crate::{modular_int::ModularInt, static_modulus::StaticMod};
        type Mint = ModularInt<StaticMod<1_000_000_007>>;
        let res = factorial_table::<Mint>(20)
            .into_iter()
            .map(|x| x.value())
            .collect::<Vec<u32>>();
        assert_eq!(
            res,
            [
                1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800,
                39916800, 479001600, 227020758, 178290591, 674358851,
                789741546, 425606191, 660911389, 557316307
            ],
        );
    }
}
