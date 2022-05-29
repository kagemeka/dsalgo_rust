pub struct Iter<T, F, I> {
    iter: I,
    f: F,
    cur: Option<T>,
}

impl<T, F, I> Iterator for Iter<T, F, I>
where
    T: Clone,
    I: Iterator<Item = T>,
    F: Fn(T, T) -> T,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_none() {
            return None;
        }
        let cur = self.cur.take().unwrap();
        self.cur = if let Some(next) = self.iter.next() {
            Some((self.f)(cur.clone(), next))
        } else {
            None
        };
        Some(cur)
    }
}

pub fn accumulate<T, F, I>(f: F, mut iter: I) -> Iter<T, F, I>
where
    T: Clone,
    F: Fn(T, T) -> T,
    I: Iterator<Item = T>,
{
    let cur = iter.next();
    Iter { iter, f, cur }
}

// TODO
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut iter = accumulate(
            |a, b| a + b,
            (0..4).into_iter(),
        );
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(6));
        assert_eq!(iter.next(), None);
    }
}
