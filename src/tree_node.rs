pub trait Parent: Sized {
    fn parent(&self) -> &Option<Self>;
}

pub trait ParentMut: Sized {
    fn parent_mut(&mut self) -> &mut Option<Self>;
}

// pub trait Get<Idx: ?Sized = usize> {
//     type Output: ?Sized;
//     fn get(&self, index: Idx) -> Self::Output;
// }

pub trait Insert {
    fn insert(self, index: usize, node: Self) -> Self;
}

impl<T> Insert for T
where
    T: crate::join::Join + crate::split::Split<usize>,
{
    fn insert(self, index: usize, node: Self) -> Self {
        let (lhs, rhs) = self.split(index);
        lhs.join(node).join(rhs)
    }
}

pub trait Pop: Sized {
    fn pop(self, index: usize) -> (Self, Self);
}

impl<T> Pop for T
where
    T: crate::join::Join + crate::split::Split<usize>,
{
    fn pop(self, index: usize) -> (Self, Self) {
        let (lhs, rhs) = self.split(index);
        let (popped, rhs) = rhs.split(1);
        (lhs.join(rhs), popped)
    }
}

pub(crate) trait Get<'a>
where
    Self: 'a,
{
    type Output;

    fn get(&'a self, index: usize) -> Self::Output;
}
