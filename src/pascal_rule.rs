pub struct PascalRule<T> {
    cache: std::collections::HashMap<u64, T>,
}

impl<T> PascalRule<T> {
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }

    /// interface is not u64 but u32
    /// to avoid key's overflow but memory overflow or infinite run time.
    /// memory overflow should not be handled here (low level API).
    pub fn calc(&mut self, n: u32, k: u32) -> T
    where
        T: std::ops::Add<Output = T> + From<usize> + Copy,
    {
        assert!(k <= n);
        // unlike combination, (n, k) is undefined if k > n
        // on pascal's triangle.
        if k == 0 {
            return 1.into();
        }
        let key = (n as u64) << 32 | k as u64;
        if !self.cache.contains_key(&key) {
            let mut v = self.calc(n - 1, k - 1);
            if k < n {
                v = v + self.calc(n - 1, k);
            }
            self.cache.insert(key, v);
        }
        *self.cache.get(&key).unwrap()
    }
}
