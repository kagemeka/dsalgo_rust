pub struct PascalRule<T>
where
    T: std::ops::Add<Output = T> + From<usize> + Clone,
{
    cache: std::collections::HashMap<usize, T>,
}

impl<T> PascalRule<T>
where
    T: std::ops::Add<Output = T> + From<usize> + Copy,
{
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }

    pub fn calc(&mut self, n: usize, k: usize) -> Result<T, ()> {
        if n < k {
            return Ok(0.into());
        }
        if k == 0 {
            return Ok(1.into());
        }
        if n >= 1 << 32 {
            return Err(());
        }
        let key = n << 32 | k;
        if !self.cache.contains_key(&key) {
            let mut v = self.calc(n - 1, k - 1)?;
            v = v + self.calc(n - 1, k)?;
            self.cache.insert(key, v);
        }
        Ok(*self.cache.get(&key).unwrap())
    }
}
