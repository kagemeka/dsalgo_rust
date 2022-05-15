pub struct WithSparseTable<'a, S> {
    first_idx: Vec<usize>,
    sp: DisjointSparseTable<'a, S>,
}
impl<'a> WithSparseTable<'a, (usize, usize)> {
    pub fn new(g: &Vec<(usize, usize)>, root: usize) -> Self {
        let (tour, first_idx, _, _, depth) = euler_tour_node(g, root);
        let sg = Semigroup::<'a, (usize, usize)> {
            op: &|x, y| std::cmp::min(*x, *y),
            commutative: true,
            idempotent: true,
        };
        let mut a = Vec::with_capacity(tour.len());
        for &i in tour.iter() {
            a.push((depth[i as usize], i as usize));
        }
        let sp = DisjointSparseTable::new(sg, &a);
        Self { first_idx: first_idx, sp: sp }
    }

    pub fn get(&self, u: usize, v: usize) -> usize {
        let mut l = self.first_idx[u];
        let mut r = self.first_idx[v];
        if l > r {
            std::mem::swap(&mut l, &mut r);
        }
        self.sp.get(l, r + 1).1
    }
}

pub struct WithSqrtDecomposition {}
