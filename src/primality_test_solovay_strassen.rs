use crate::solovay_strassen_fixed_bases::SolovayStrassenFixedBases;

pub fn solovay_strassen_test(n: u64, epochs: u8) -> bool {
    let tester = SolovayStrassenFixedBases::from_random_bases(epochs);
    tester.is_prime(n)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
