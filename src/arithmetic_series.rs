use crate::sum_arithmetic_progression::sum_arithmetic_progression;

pub fn arithmetic_series(diff: i64, a_0: i64, nth: u64) -> i64 {
    sum_arithmetic_progression(
        a_0,
        a_0 + diff * nth as i64,
        nth + 1,
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::*;
        assert_eq!(arithmetic_series(1, 1, 9), 55);
        // 9th term is 1 + 1 * 9 = 10
    }
}
