use crate::{
    miller_rabin_fixed_bases::miller_rabin_fixed_bases,
    vector_unique::vector_unique,
};

pub fn miller_rabin_random_bases(n: u64, epochs: u8) -> bool {
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let bases = (0..epochs)
        .map(|_| rng.gen_range(2..n - 1))
        .collect::<Vec<_>>();
    miller_rabin_fixed_bases(&vector_unique(bases), n)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert!(!miller_rabin_random_bases(
            512461, 20
        ),);
    }
}
