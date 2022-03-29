pub trait Split: Sized {
    fn split(self, index: usize) -> (Self, Self);
}
