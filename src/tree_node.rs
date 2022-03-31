pub trait Parent: Sized {
    fn parent(&self) -> &Option<Self>;
}

pub trait ParentMut: Sized {
    fn parent_mut(&mut self) -> &mut Option<Self>;
}

pub trait Get<Idx: ?Sized = usize> {
    type Output: ?Sized;
    fn get(&self, index: Idx) -> Self::Output;
}
