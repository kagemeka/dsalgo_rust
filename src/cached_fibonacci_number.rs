pub struct CachedFibonacci<T> {
    fib: Vec<Option<T>>,
}

impl<T> CachedFibonacci<T> {
    pub fn new() -> Self { Self { fib: vec![] } }

    pub fn calc(&mut self, n: usize) -> T
    where
        T: From<u64> + Clone + std::ops::Add<Output = T>,
    {
        if self.fib.len() <= n {
            self.fib.extend(vec![
                None;
                n + 1 - self.fib.len()
            ]);
        }
        if self.fib[n].is_some() {
            return self.fib[n].clone().unwrap();
        }
        match n {
            0 => self.fib[0] = Some(0.into()),
            1 => self.fib[1] = Some(1.into()),
            _ => {
                self.fib[n] = Some(self.calc(n - 1).clone() + self.calc(n - 2));
            },
        }
        self.fib[n].clone().unwrap()
    }
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
