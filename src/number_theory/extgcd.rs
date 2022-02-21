pub fn extgcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (g, s, t) = extgcd(b, a % b);
    (g, t, s - a / b * t)
}
