pub trait Join<Rhs = Self> {
    fn join(self, rhs: Rhs) -> Self;
}
