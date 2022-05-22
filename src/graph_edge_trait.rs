pub trait Edge {}

pub trait UndirectedEdge {
    type V;
    fn u(&self) -> &Self::V;
    fn v(&self) -> &Self::V;
}

pub trait ToDirected {
    type E;
    fn to_directed(self) -> Self::E;
}

pub trait From {
    type V;
    fn from(&self) -> &Self::V;
}

pub trait To {
    type V;

    fn to(&self) -> &Self::V;
}

pub trait DirectedEdge: To {}

pub trait Reversed {
    fn reversed(self) -> Self;
}

pub trait Value {
    type T;
    fn value(&self) -> &Self::T;
}
pub trait ValueMut {
    type T;
    fn value_mut(&mut self) -> &mut Self::T;
}

pub trait Weight<T> {
    fn weight(&self) -> &T;
}
pub trait WeightMut<T> {
    fn weight_mut(&mut self) -> &mut T;
}

pub trait Capacity<T> {
    fn capacity(&self) -> &T;
}
