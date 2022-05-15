/// ```
/// use dsalgo::bit_shr_until_odd::bit_shr_until_odd;
/// assert_eq!(bit_shr_until_odd(0), Err(()));
/// assert_eq!(bit_shr_until_odd(1), Ok(1));
/// assert_eq!(bit_shr_until_odd(2), Ok(1));
/// assert_eq!(bit_shr_until_odd(12), Ok(3));
/// ```
pub fn bit_shr_until_odd(n: u64) -> Result<u64, ()> {
    if n == 0 { Err(()) } else { Ok(n >> n.trailing_zeros()) }
}
