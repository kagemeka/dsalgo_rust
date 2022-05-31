use crate::prime_factorize_with_lpf::PrimeFactorizeLPF;

pub fn prime_factorize_factorial_lpf(n: u64) -> Vec<u64> {
    let size = n as usize + 1;
    let mut count = vec![0; size];
    let lpf = PrimeFactorizeLPF::new(size);
    for i in 2..=n as u64 {
        for (p, e) in lpf.factorize(i) {
            count[p as usize] += e;
        }
    }
    count
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
