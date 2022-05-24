// TODO
// what's this?

// get_labels methods have same implementation in union-find,
// potentialized-union-find. so we should define UnionFind trait.

pub trait FindRoot {
    fn find_root(&mut self, u: usize) -> usize;
}

pub trait Unite {
    fn unite(&mut self, u: usize, v: usize);
}

pub trait SizeOf {
    fn size_of(&self, u: usize) -> usize;
}

pub trait UnionFind {}

impl<T> UnionFind for T where T: FindRoot + Unite {}
