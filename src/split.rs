pub trait Split<Idx: ?Sized>: Sized {
    fn split(self, index: Idx) -> (Self, Self);
}
