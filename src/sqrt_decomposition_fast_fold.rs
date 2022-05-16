use crate::{monoid::Monoid, sqrt_decomposition::SqrtDecomposition};

impl<S, Id, M> SqrtDecomposition<S, Id, M>
where
    M: Monoid<S, Id>,
    S: Clone,
{
    /// faster than normal fold by constant time optimization.
    pub fn fast_fold(&self, l: usize, r: usize) -> S {
        assert!(l <= r && r <= self.size());
        let n = self.sqrt();
        let mut v = M::identity();
        let lj = (l + n - 1) / n;
        let rj = r / n;
        if rj < lj {
            (l..r).for_each(|i| {
                v = M::operate(v.clone(), self.data[i].clone());
            });
            return v;
        }
        (l..lj * n).for_each(|i| {
            v = M::operate(v.clone(), self.data[i].clone());
        });
        (lj..rj).for_each(|j| {
            v = M::operate(
                v.clone(),
                self.buckets[j].clone(),
            );
        });
        (rj * n..r).for_each(|i| {
            v = M::operate(v.clone(), self.data[i].clone());
        });
        v
    }
}
