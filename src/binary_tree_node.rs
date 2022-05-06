pub trait Child: Sized {
    fn left(&self) -> &Option<Self>;
    fn right(&self) -> &Option<Self>;
}

pub trait ChildMut: Child {
    fn left_mut(&mut self) -> &mut Option<Self>;
    fn right_mut(&mut self) -> &mut Option<Self>;
}

pub trait Update {
    fn update(&mut self);
}

// use crate::identifier;

// pub struct WithParent;

// impl identifier::Identifier for WithParent {}

pub(crate) trait Rotation {
    fn rotate_left(self) -> Self;
    fn rotate_right(self) -> Self;
}

/// for updating node data when the childs are changed.
pub(crate) trait UpdateData {
    fn update(&mut self, left_data: &Self, right_data: &Self);
}
