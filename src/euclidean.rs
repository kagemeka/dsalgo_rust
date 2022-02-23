pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn lcm(a: usize, b: usize) -> usize { a / gcd(a, b) * b }

pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, s, t) = extgcd(b, a % b);
    (g, t, s - a / b * t)
}
