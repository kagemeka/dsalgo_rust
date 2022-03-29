pub trait Pop: Sized {
    type Data;
    fn pop(&mut self, index: usize) -> Self::Data;
}
