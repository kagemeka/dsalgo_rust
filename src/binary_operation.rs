pub trait BinaryOperation<Lhs, Rhs, Codomain, Id> {
    fn operate(lhs: Lhs, rhs: Rhs) -> Codomain;
}
