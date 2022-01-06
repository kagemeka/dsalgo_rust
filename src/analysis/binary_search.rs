pub fn bisect<T>(is_ok: &dyn Fn(&T) -> bool, a: &[T]) -> usize {
    let mut lo = 0;
    let mut hi = a.len();
    while lo < hi {
        let i = (lo + hi) >> 1;
        if is_ok(&a[i]) { hi = i; } else { lo = i + 1; }        
    }
    lo
}

pub fn lower_bound<T: Ord>(a: &[T], x: &T) -> usize {
    let is_ok = |y: &T| y >= x;
    bisect(&is_ok, a)
}