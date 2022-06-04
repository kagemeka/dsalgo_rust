use crate::{
    euler_criterion::try_euler_criterion,
    jacobi_symbol::jacobi_symbol,
};

pub fn is_composite_euler_jacobi(base: u64, n: u64) -> bool {
    assert!(n > 2 && n & 1 == 1 && 2 <= base && base < n);
    // 2 <= a because if a == 1, it's trivial jacobi = euler = 1.
    // compare jcobi symbol and euler's criterion.
    let jacobi = jacobi_symbol(n, base);
    if jacobi == 0 {
        return true;
    }
    if let Ok(euler) = try_euler_criterion(n, base) {
        let jacobi = if jacobi == 1 { 1 } else { n - 1 };
        euler != jacobi
    } else {
        true
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
