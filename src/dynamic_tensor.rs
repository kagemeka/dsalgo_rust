#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynamicTensor<T> {
    shape: Vec<usize>,
    data: Vec<T>,
}

impl<T> DynamicTensor<T> {
    pub fn shape(&self) -> &[usize] { &self.shape }

    pub(crate) fn dim(&self) -> usize { self.shape.len() }

    pub(crate) fn strides(&self) -> Vec<usize> {
        let mut strides: Vec<usize> = self.shape.clone();
        let d = self.dim();
        if d > 0 {
            strides[d - 1] = 1;
        }
        for i in (1..d).rev() {
            strides[i - 1] = strides[i] * self.shape[i];
        }
        strides
    }

    pub(crate) fn compute_size(shape: &[usize]) -> usize {
        let mut size = 1;
        for &dim in shape {
            size *= dim;
        }
        size
    }

    pub fn size(&self) -> usize { self.data.len() }
}

impl<T: Default> DynamicTensor<T> {
    pub fn new(shape: &[usize]) -> Self {
        let size = Self::compute_size(&shape);
        Self {
            shape: shape.to_vec(),
            data: (0..size).map(|_| T::default()).collect(),
        }
    }
}

impl<T> DynamicTensor<T> {
    fn flatten_index(&self, index: &[usize]) -> usize {
        let mut idx = 0;
        let strides = self.strides();
        assert_eq!(index.len(), self.dim());
        for i in 0..self.dim() {
            idx += strides[i] * index[i];
        }
        idx
    }
}

impl<T> std::ops::Index<&[usize]> for DynamicTensor<T> {
    type Output = T;

    fn index(&self, index: &[usize]) -> &Self::Output {
        &self.data[self.flatten_index(index)]
    }
}

impl<T> std::ops::IndexMut<&[usize]> for DynamicTensor<T> {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        let idx = self.flatten_index(index);
        &mut self.data[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut a = DynamicTensor::<i64>::new(&[1, 2, 3]);
        a[&[0, 0, 0]] = 1;
        assert_eq!(a[&[0, 0, 0]], 1);
        println!("{:?}", a);

        let b = DynamicTensor::<i64>::new(&[]);
        println!("{:?}", b);
    }
}
