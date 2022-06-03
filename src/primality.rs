pub trait Primality {}

pub(crate) fn is_trivial_composite(n: usize) -> bool { n > 2 && n & 1 == 0 }

// def miller_test() -> bool:
//     ...

// def miller_rabin_solovay_strassen_test() -> bool:
//     ...

// def lucas_test() -> bool:
//     ...

// def lucas_lehmer_test() -> bool:
//     ...

// def lucas_lehmer_reisel_test() -> bool:
//     ...

// def packlington_test() -> bool:
//     ...

// def frobenius_test() -> bool:
//     ...

// def baillie_psw_test() -> bool:
//     ...

// def agrawal_kayal_saxena_test(n: int) -> bool:
//     ...

// def pollard_rho() -> None:
