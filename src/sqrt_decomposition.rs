use crate::{floor_sqrt::floor_sqrt, monoid::Monoid, semigroup::Semigroup};

pub struct SqrtDecomposition<G: Semigroup<Id>, Id> {
    pub(crate) data: Vec<G::S>,
    pub(crate) buckets: Vec<G::S>,
}

impl<G: Semigroup<Id>, Id> SqrtDecomposition<G, Id> {
    pub fn size(&self) -> usize { self.data.len() }

    pub(crate) fn sqrt(&self) -> usize {
        let n = self.buckets.len();
        (self.size() + n - 1) / n
    }
}

impl<G, Id> std::iter::FromIterator<G::S> for SqrtDecomposition<G, Id>
where
    G: Semigroup<Id>,
    G::S: Clone,
{
    fn from_iter<T: IntoIterator<Item = G::S>>(iter: T) -> Self {
        let data = iter.into_iter().collect::<Vec<_>>();
        let size = data.len();
        let n = floor_sqrt(size as u64) as usize;
        let buckets = (0..(size + n - 1) / n)
            .map(|j| {
                // data[j * n..std::cmp::min((j + 1) * n, size)]
                //     .iter()
                //     .cloned()
                //     .reduce(|l, r| G::operate(l, r))
                //     .unwrap()
                // CHANGE LATER: reduce is not supported on atcoder yet.

                let mut iter = data[j * n..std::cmp::min((j + 1) * n, size)]
                    .iter()
                    .cloned();
                let mut v = iter.next().unwrap();
                for x in iter {
                    v = G::operate(v, x);
                }
                v
            })
            .collect();
        Self { data, buckets }
    }
}

impl<G, Id> SqrtDecomposition<G, Id>
where
    G: Semigroup<Id>,
    G::S: Clone,
{
    pub(crate) fn update(&mut self, bucket: usize) {
        let j = bucket;
        let n = self.sqrt();
        // self.buckets[j] = self.data
        //     [j * n..std::cmp::min((j + 1) * n, self.size())]
        //     .iter()
        //     .cloned()
        //     .reduce(|l, r| G::operate(l, r))
        //     .unwrap();
        // CHANGE LATER: reduce is not supported on atcoder yet.
        let mut iter = self.data
            [j * n..std::cmp::min((j + 1) * n, self.size())]
            .iter()
            .cloned();
        let mut v = iter.next().unwrap();
        for x in iter {
            v = G::operate(v, x);
        }
        self.buckets[j] = v;
    }

    pub fn apply<F>(&mut self, i: usize, f: F)
    where
        F: FnOnce(G::S) -> G::S,
    {
        self.data[i] = f(self.data[i].clone());
        self.update(i / self.sqrt());
    }

    // TODO: move out from core implementation.
    // because set can be defined as application of 'replacement'
    // (the core is apply method)
    pub fn set(&mut self, i: usize, x: G::S) {
        self.data[i] = x;
        self.update(i / self.sqrt());
    }

    pub fn reduce(&self, l: usize, r: usize) -> G::S {
        assert!(l < r && r <= self.size());
        // just for early panic. it's not necessary to be checked here.
        let n = self.sqrt();
        // (0..self.buckets.len())
        //     .filter_map(|j| {
        //         if r <= n * j || n * (j + 1) <= l {
        //             return None;
        //         }
        //         if l <= n * j && n * (j + 1) <= r {
        //             return Some(self.buckets[j].clone());
        //         }
        //         (0..n)
        //             .filter_map(|k| {
        //                 let i = j * n + k;
        //                 if l <= i && i < r {
        //                     Some(self.data[i].clone())
        //                 } else {
        //                     None
        //                 }
        //             })
        //             .reduce(|l, r| G::operate(l, r))
        //     })
        //     .reduce(|l, r| G::operate(l, r))
        // CHANGE LATER: reduce is not supported on atcoder yet.

        let mut iter = (0..self.buckets.len()).filter_map(|j| {
            if r <= n * j || n * (j + 1) <= l {
                return None;
            }
            if l <= n * j && n * (j + 1) <= r {
                return Some(self.buckets[j].clone());
            }

            let mut iter = (0..n).filter_map(|k| {
                let i = j * n + k;
                if l <= i && i < r { Some(self.data[i].clone()) } else { None }
            });
            let mut v = iter.next().unwrap();
            for x in iter {
                v = G::operate(v, x);
            }
            Some(v)
        });
        let mut v = iter.next().unwrap();
        for x in iter {
            v = G::operate(v, x);
        }
        v
    }
}
