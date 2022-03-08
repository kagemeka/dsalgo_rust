fn is_trivial_composite(n: usize) -> bool { n > 2 && n & 1 == 0 }

// TODO need dynamic modular for storing modulus `n`.
// fn is_composite<M: crate::modular_static::Modulus>(
//     n: usize,
//     base: crate::modular_static::Modular<M>,
// ) -> bool {
//     if is_trivial_composite(n) {
//         return false;
//     }
//     let (mut s, mut d) = (0, n - 1);
//     while d & 1 == 0 {
//         s += 1;
//         d >>= 1;
//     }
//     let mut x = base.pow(d);
//     // n - 1 = 2^s*d
// }
// def _is_composite(n: int, base: int) -> bool:
//     assert n >= 3
//     r, d = 0, n - 1
//     while d & 1 == 0:
//         r += 1
//         d >>= 1
//     # n - 1 = d2^r
//     x = pow(base, d, n)
//     for _ in range(r):
//         y = x * x % n
//         if y == 1:
//             return x != 1 and x != n - 1
//         x = y
//     return True

// def _miller_rabin_fixed_bases(n: int, bases: list[int]) ->
// bool:     assert n >= 1
//     if _is_trivial_composite(n):
//         return False
//     if n == 2:
//         return True
//     for base in bases:
//         if _is_composite(n, base):
//             return False
//     return True

// def miller_rabin_test(n: int, check_times: int = 20) ->
// bool:     assert n >= 1
//     if n == 1:
//         return False
//     bases = list(set(random.randint(1, n - 1) for _ in
// range(check_times)))     return _miller_rabin_fixed_bases(n,
// bases)

// def miller_rabin_test_32(n: int) -> bool:
//     BASES = [2, 7, 61]
//     return _miller_rabin_fixed_bases(n, BASES)

// def miller_rabin_test_64(n: int) -> bool:
//     BASES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37]
//     return _miller_rabin_fixed_bases(n, BASES)

// def miller_rabin_test_64_v2(n: int) -> bool:
//     BASES = [2, 325, 9375, 28178, 450775, 9780504,
// 1795265022]     return _miller_rabin_fixed_bases(n, BASES)
