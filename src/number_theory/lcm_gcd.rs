pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

pub fn lcm(a: usize, b: usize) -> usize { a / gcd(a, b) * b }
