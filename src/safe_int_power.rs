pub fn safe_int_pow(x: u64, exponent: u8) -> Result<u64, &'static str> {
    if exponent == 0 {
        return Ok(1);
    }
    let mut y = safe_int_pow(x, exponent >> 1)?;
    debug_assert!(y > 0);
    const M: u64 = std::u64::MAX;
    if y > M / y {
        return Err("overflow");
    }
    y *= y;
    if exponent & 1 == 1 {
        if y > M / x {
            return Err("overflow");
        }
        y *= x;
    }
    Ok(y)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert!(safe_int_pow(2, 0).is_ok());
        assert!(safe_int_pow(2, 1).is_ok());
        assert!(safe_int_pow(2, 63).is_ok());
        assert!(safe_int_pow(2, 64).is_err());
    }
}
