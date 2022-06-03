use crate::euclidean_algorithms::euclidean_mod_gcd_inv;

pub fn modular_inverse_extgcd(
    modulus: u64,
    element: u64,
) -> Result<u64, &'static str> {
    let (gcd, inv) = euclidean_mod_gcd_inv(modulus, element);
    if gcd == 1 {
        Ok(inv)
    } else {
        Err("modulus and element are not coprime")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
