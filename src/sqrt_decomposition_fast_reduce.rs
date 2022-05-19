use crate::{semigroup::Semigroup, sqrt_decomposition::SqrtDecomposition};

impl<S, G, Id> SqrtDecomposition<S, G, Id>
where
    G: Semigroup<S, Id>,
    S: Clone,
{
    /// faster with constant time optimization.
    pub fn fast_reduce(&self, mut l: usize, r: usize) -> S {
        assert!(l < r && r <= self.size());
        let n = self.sqrt();
        let mut v = self.data[l].clone();
        l += 1;
        let lj = (l + n - 1) / n;
        let rj = r / n;
        if rj < lj {
            for i in l..r {
                v = G::operate(v, self.data[i].clone());
            }
            return v;
        }
        for i in l..lj * n {
            v = G::operate(v, self.data[i].clone());
        }
        for j in lj..rj {
            v = G::operate(v, self.buckets[j].clone());
        }
        for i in rj * n..r {
            v = G::operate(v, self.data[i].clone());
        }
        v
    }
}
