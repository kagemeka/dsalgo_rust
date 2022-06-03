use crate::miller_rabin_is_precise_composite::is_precise_composite;

pub fn miller_rabin_fixed_bases(bases: &[u64], n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n < 2 || n & 1 == 0 {
        return false;
    }
    bases
        .iter()
        .map(|&base| base % n)
        .filter(|base| base != &0)
        .all(|b| !is_precise_composite(b, n))
    // strong probable prime.
}
// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
