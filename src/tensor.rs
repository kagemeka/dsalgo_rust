use crate::tensor_property::{Shape, Size, Strides};

// TODO: define macro for initialization.
// accept scalar, 1d vec, 2d vec, ... n-d vec.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tensor<P, T> {
    _phantom: std::marker::PhantomData<P>,
    data: Vec<T>,
}

impl<P, T> Shape for Tensor<P, T>
where
    P: Shape,
{
    fn shape() -> &'static [usize] { P::shape() }
}

impl<P, T> Default for Tensor<P, T>
where
    P: Size,
    T: Default,
{
    fn default() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
            data: (0..P::size()).map(|_| T::default()).collect(),
        }
    }
}

impl<P, T> Tensor<P, T>
where
    P: Size,
    T: Default,
{
    pub fn new() -> Self { Self::default() }
}

impl<P, T> From<Vec<T>> for Tensor<P, T>
where
    P: Size,
{
    fn from(v: Vec<T>) -> Self {
        assert_eq!(v.len(), P::size());
        Self {
            _phantom: std::marker::PhantomData,
            data: v,
        }
    }
}

impl<P, T> Tensor<P, T>
where
    P: Strides,
{
    fn flatten_index(&self, index: &[usize]) -> usize {
        let mut idx = 0;
        let strides = P::strides();
        assert!(index.len() == strides.len());
        for i in 0..index.len() {
            idx += index[i] * strides[i];
        }
        idx
    }
}

impl<P, T> std::ops::Index<&[usize]> for Tensor<P, T>
where
    P: Strides,
{
    type Output = T;

    fn index(&self, index: &[usize]) -> &Self::Output {
        let idx = self.flatten_index(index);
        &self.data[idx]
    }
}

impl<P, T> std::ops::IndexMut<&[usize]> for Tensor<P, T>
where
    P: Strides,
{
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let idx = self.flatten_index(index);
        &mut self.data[idx]
    }
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
