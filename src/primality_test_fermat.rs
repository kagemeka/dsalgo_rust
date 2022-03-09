fn is_composite<M: crate::modular_static::Modulus + Copy>(
    n: usize,
    base: crate::modular_static::Modular<M>,
) -> bool {
    use crate::euclidean::greatest_common_divisor;
    if crate::primality::is_trivial_composite(n) {
        return true;
    }
    if greatest_common_divisor(n, base.value()) != 1 {
        return true;
    }
    if base.pow(n - 1).value() != 1 {
        return true;
    }
    false
}

pub fn fermat_test(n: usize, check_times: usize) -> bool {
    if n < 2 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    #[derive(Clone, Copy)]
    struct Mod;
    use crate::{
        modular_runtime_static::{RuntimeModulus, RuntimeModulusCore},
        modular_static::{Modular, Modulus},
    };
    impl RuntimeModulus for Mod {
        fn core() -> &'static RuntimeModulusCore {
            static MODULUS: RuntimeModulusCore = RuntimeModulusCore::new(1);
            &MODULUS
        }
    }
    impl Modulus for Mod {
        fn value() -> usize { Self::core().get_value() }
    }
    Mod::set(n);
    type Mint = Modular<Mod>;
    use rand::{thread_rng, Rng};
    let mut rng = thread_rng();
    let mut bases = (0..check_times)
        .map(|_| rng.gen_range(2..n - 1))
        .collect::<Vec<_>>();
    bases = crate::vector_util::unique(&bases);
    for base in bases.iter().map(|x| Mint::new(*x)) {
        if is_composite(n, base) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(super::fermat_test(998_244_353, 100), true);
        assert_eq!(super::fermat_test(1_000_000_007, 100), true);
        assert_eq!(super::fermat_test(561, 100), false);
        assert_eq!(super::fermat_test(512461, 100), false);
    }
}
