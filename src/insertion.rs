pub trait Insert {
    type Data;
    fn insert(&mut self, index: usize, data: Self::Data);
}
