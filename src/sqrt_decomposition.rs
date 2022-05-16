use crate::{floor_sqrt::floor_sqrt, monoid::Monoid};

pub struct SqrtDecomposition<S, Id, M>
where
    M: Monoid<S, Id>,
{
    phantom_id: std::marker::PhantomData<Id>,
    phantom_m: std::marker::PhantomData<M>,
    pub(crate) data: Vec<S>,
    pub(crate) buckets: Vec<S>,
}

impl<S, Id, M> std::iter::FromIterator<S> for SqrtDecomposition<S, Id, M>
where
    M: Monoid<S, Id>,
    S: Clone,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        let data = iter.into_iter().collect::<Vec<_>>();
        let size = data.len();
        let n = floor_sqrt(size as u64) as usize;
        let mut buckets = vec![M::identity(); (size + n - 1) / n];
        (0..size).for_each(|i| {
            let j = i / n;
            buckets[j] = M::operate(
                buckets[j].clone(),
                data[i].clone(),
            );
        });
        Self {
            phantom_id: std::marker::PhantomData,
            phantom_m: std::marker::PhantomData,
            data,
            buckets,
        }
    }
}
impl<S, Id, M> SqrtDecomposition<S, Id, M>
where
    M: Monoid<S, Id>,
{
    pub fn size(&self) -> usize { self.data.len() }

    pub(crate) fn sqrt(&self) -> usize {
        let n = self.buckets.len();
        (self.size() + n - 1) / n
    }
}

impl<S, Id, M> SqrtDecomposition<S, Id, M>
where
    M: Monoid<S, Id>,
    S: Clone,
{
    pub fn set(&mut self, i: usize, x: S) {
        assert!(i < self.size());
        let n = self.sqrt();
        self.data[i] = x;
        let j = i / n;
        self.buckets[j] = M::identity();
        let size = self.size();
        (0..n).filter(|k| j * n + k < size).for_each(|k| {
            self.buckets[j] = M::operate(
                self.buckets[j].clone(),
                self.data[j * n + k].clone(),
            );
        });
    }

    pub fn fold(&self, l: usize, r: usize) -> S {
        assert!(l <= r && r <= self.size());
        let n = self.sqrt();
        let mut v = M::identity();
        (0..self.buckets.len())
            .filter(|&j| l < n * (j + 1) && n * j < r)
            .for_each(|j| {
                if l <= n * j && n * (j + 1) <= r {
                    v = M::operate(
                        v.clone(),
                        self.buckets[j].clone(),
                    );
                } else {
                    (0..n)
                        .filter(|&k| {
                            let i = j * n + k;
                            l <= i && i < r
                        })
                        .for_each(|k| {
                            v = M::operate(
                                v.clone(),
                                self.data[j * n + k].clone(),
                            );
                        });
                }
            });
        v
    }
}

// TODO: set_range (lazy sqrt decomp)
