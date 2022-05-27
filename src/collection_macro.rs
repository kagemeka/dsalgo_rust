#[allow(unused_macros)]
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$(($k, $v),)*]))
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        use std::iter::{Iterator, IntoIterator};
        Iterator::collect(IntoIterator::into_iter([$($v,)*]))
    }};
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    #[test]
    fn test() {
        let a: BTreeMap<String, usize> =
            collection! { "a".to_string() => 1, "b".to_string() => 2 };
        let mut b = BTreeMap::new();
        b.insert("a".to_string(), 1);
        b.insert("b".to_string(), 2);
        assert_eq!(a, b);
    }
}
