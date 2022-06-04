use crate::euler_criterion::euler_criterion;

/// for prime modulus p and a \in \Z/p\Z
pub fn is_quadratic_residue_z_pz_euler_criterion(p: u64, a: u64) -> bool {
    euler_criterion(p, a) == 1
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
