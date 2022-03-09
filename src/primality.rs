pub fn is_prime(n: usize) -> bool { crate::divisor::find_divisors(n).len() == 2 }

pub fn is_prime_table(size: usize) -> Vec<bool> {
    crate::sieve_of_eratosthenes::sieve_of_eratosthens(size)
}

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

// def fermat_test(n: int, check_times: int = 100) -> bool:
//     assert n >= 1
//     if n == 1:
//         return False
//     if n == 2:
//         return True

//     def n_is_composite(base: int) -> bool:
//         nonlocal n
//         if math.gcd(n, base) != 1:
//             return True
//         if pow(base, n - 1, n) != 1:
//             return True
//         return False

//     checked_bases = set()

//     for _ in range(check_times):
//         base = random.randint(2, n - 1)
//         if base in checked_bases:
//             continue
//         if n_is_composite(base):  # the base is called
// witness.             return False
//         checked_bases.add(base)

//     # might be pseudo prime like Carmichael number.
//     # if not prime actually, each checked base is called
// liar.     return True

// CARMICHAEL_NUMBERS: typing.Final[list[int]] = [
//     561,
//     1105,
//     1729,
//     2465,
//     2821,
//     6601,
//     8911,
//     10585,
//     15841,
//     29341,
//     41041,
//     46657,
//     52633,
//     62745,
//     63973,
//     75361,
//     101101,
//     115921,
//     126217,
//     162401,
//     172081,
//     188461,
//     252601,
//     278545,
//     294409,
//     314821,
//     334153,
//     340561,
//     399001,
//     410041,
//     449065,
//     488881,
//     512461,
// ]

// def pollard_rho() -> None:
