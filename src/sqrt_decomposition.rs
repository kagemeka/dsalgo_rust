use crate::{floor_sqrt::floor_sqrt, monoid::Monoid, semigroup::Semigroup};

// TODO: use Semigroup2 after language update on AtCoder
pub struct SqrtDecomposition<S, G, Id> {
    phantom: std::marker::PhantomData<(G, Id)>,
    pub(crate) data: Vec<S>,
    pub(crate) buckets: Vec<S>,
}

impl<S, G, Id> SqrtDecomposition<S, G, Id> {
    pub fn size(&self) -> usize { self.data.len() }

    pub(crate) fn sqrt(&self) -> usize {
        let n = self.buckets.len();
        (self.size() + n - 1) / n
    }
}

impl<S, G, Id> std::iter::FromIterator<S> for SqrtDecomposition<S, G, Id>
where
    G: Semigroup<S, Id>,
    S: Clone,
{
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
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
        Self {
            phantom: std::marker::PhantomData,
            data,
            buckets,
        }
    }
}

impl<S, G, Id> SqrtDecomposition<S, G, Id>
where
    G: Semigroup<S, Id>,
    S: Clone,
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
        F: FnOnce(&S) -> S,
    {
        self.data[i] = f(&self.data[i]);
        self.update(i / self.sqrt());
    }

    // TODO: move out from core implementation. (the core is apply method)
    /// set is defined as the application of 'replacement'
    pub fn set(&mut self, i: usize, x: S) {
        self.apply(i, |_| x);
        self.update(i / self.sqrt());
    }

    pub fn reduce(&self, l: usize, r: usize) -> S {
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
