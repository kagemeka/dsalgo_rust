use crate::least_common_multiple::lcm;

pub fn lcm_reduce<I>(iter: I) -> u64
where
    I: Iterator<Item = u64>,
{
    iter.fold(1, |a, b| lcm(a, b))
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
