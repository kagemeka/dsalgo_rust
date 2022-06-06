use crate::{
    fermat_factorization_method::fermat_factorization_method,
    make_sparse_histogram::make_sparse_histogram,
};

fn prime_factorize_flat_fermat(mut n: u64) -> Vec<u64> {
    assert!(n > 0);
    let mut res = vec![];
    let ctz = n.trailing_zeros();
    res.extend(vec![2; ctz as usize]);
    n >>= ctz;
    if n == 1 {
        return res;
    }
    let a = fermat_factorization_method(n);
    let b = n / a;
    debug_assert!(a >= b && a * b == n);
    if a == n {
        res.push(a);
    } else {
        res.extend(prime_factorize_flat_fermat(a).into_iter());
        res.extend(prime_factorize_flat_fermat(b).into_iter());
    }
    res
}

pub fn prime_factorize_fermat(n: u64) -> Vec<(u64, u8)> {
    make_sparse_histogram(prime_factorize_flat_fermat(n))
        .into_iter()
        .map(|(p, c)| (p, c as u8))
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        use crate::prime_factorize_trial_division::*;
        for i in 1..100 {
            assert_eq!(
                prime_factorize_fermat(i),
                prime_factorize_trial_division(i)
            );
        }
    }
}
