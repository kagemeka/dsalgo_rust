pub trait Parent: Sized {
    fn parent(&self) -> &Option<Self>;
}

pub trait ParentMut: Sized {
    fn parent_mut(&mut self) -> &mut Option<Self>;
}
