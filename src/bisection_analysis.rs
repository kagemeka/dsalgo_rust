/// generalized bisection method in analysis.
pub fn bisect<T, B, F>(
    calc_middle: &B,
    is_ok: &F,
    trivial_ng: T,
    trivial_ok: T,
    max_epohcs: usize,
) -> T
where
    B: Fn(T, T) -> T,
    F: Fn(&T) -> bool,
    T: Clone + PartialEq,
{
    let mut ng = trivial_ng;
    let mut ok = trivial_ok;
    for _ in 0..max_epohcs {
        let middle = calc_middle(ng.clone(), ok.clone());
        if middle == ng || middle == ok {
            break;
        }
        if is_ok(&middle) {
            ok = middle;
        } else {
            ng = middle;
        }
    }
    ok
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
