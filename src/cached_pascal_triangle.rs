pub struct CachedPascalTriangle<T>
where
    T: std::ops::Add<Output = T> + From<usize> + Copy,
{
    cache: std::collections::HashMap<usize, T>,
}

impl<T> CachedPascalTriangle<T>
where
    T: std::ops::Add<Output = T> + From<usize> + Copy,
{
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }

    pub fn calc(&mut self, n: usize, k: usize) -> T {
        if n < k {
            return 0.into();
        }
        if k == 0 {
            return 1.into();
        }
        let key = n << 32 | k;
        if !self.cache.contains_key(&key) {
            let mut v = self.calc(n - 1, k - 1);
            v = v + self.calc(n - 1, k);
            self.cache.insert(key, v);
        }
        *self.cache.get(&key).unwrap()
    }
}
