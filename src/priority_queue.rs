pub trait PriorityQueue {}

pub trait MinimumQueue {}
pub trait MaximumQueue {}

pub trait Push {
    type T;

    fn push(&mut self, x: Self::T);
}

pub trait Pop {
    type T;

    fn pop(&mut self) -> Option<Self::T>;
}

pub trait Top {
    type T;

    fn top(&mut self) -> &Self::T;
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}
