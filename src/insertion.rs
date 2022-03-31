pub trait Insert<Idx: ?Sized = usize> {
    type Data;
    fn insert(&mut self, index: Idx, data: Self::Data);
}
