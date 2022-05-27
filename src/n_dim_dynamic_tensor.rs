#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NdimDynamicTensor<T, const D: usize> {
    shape: [usize; D],
    data: Vec<T>,
}

// impl<T: std::fmt::Debug, const NDIM: usize> std::fmt::Debug
//     for NdimDynamicTensor<T, NDIM>
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Tensor")
//             .field("shape", &self.shape)
//             .field("data", &self.data)
//             .finish()
//     }
// }

impl<T, const D: usize> NdimDynamicTensor<T, D> {
    pub const fn shape(&self) -> &[usize; D] { &self.shape }

    pub fn strides(&self) -> [usize; D] {
        let mut strides: [usize; D] = self.shape.into();
        if D > 0 {
            strides[D - 1] = 1;
        }
        for i in (1..D).rev() {
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

impl<T: Default, const D: usize> NdimDynamicTensor<T, D> {
    pub fn new(shape: [usize; D]) -> Self {
        Self {
            shape,
            data: (0..Self::compute_size(&shape))
                .map(|_| T::default())
                .collect(),
        }
    }
}

impl<T, const D: usize> NdimDynamicTensor<T, D> {
    fn flatten_index(&self, index: [usize; D]) -> usize {
        let mut idx = 0;
        let strides = self.strides();
        for i in 0..D {
            idx += index[i] * strides[i];
        }
        idx
    }
}

impl<T, const D: usize> std::ops::Index<[usize; D]>
    for NdimDynamicTensor<T, D>
{
    type Output = T;

    fn index(&self, index: [usize; D]) -> &Self::Output {
        &self.data[self.flatten_index(index)]
    }
}

impl<T, const D: usize> std::ops::IndexMut<[usize; D]>
    for NdimDynamicTensor<T, D>
{
    fn index_mut(&mut self, index: [usize; D]) -> &mut Self::Output {
        let idx = self.flatten_index(index);
        &mut self.data[idx]
    }
}

// TODO: move to matrix_dot_product.rs
impl<T> std::ops::Mul for NdimDynamicTensor<T, 2>
where
    T: Copy + Default + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        assert_eq!(self.shape[1], rhs.shape[0]);
        let h = self.shape[0];
        let w = rhs.shape[1];
        let n = self.shape[1];
        let mut res = Self::new([h, w]);
        for i in 0..h {
            for j in 0..w {
                for k in 0..n {
                    res[[i, j]] = res[[i, j]] + self[[i, k]] * rhs[[k, j]];
                }
            }
        }
        res
    }
}

// impl<T: Copy + Default + group_theory::Semiring>
// Tensor<T, 2> {     pub fn e(&self) -> Self {
//         let (h, w) = (self.shape[0], self.shape[1]);
//         let mut e = Self::new(self.shape);
//         for i in 0..h {
//             for j in 0..w {
//                 e[[i, j]] =
// self::group_theory::AddIdentity::identity();
// }         }
//         for i in 0..h {
//             e[[i, i]] =
// self::group_theory::MulIdentity::identity();         }
//         e
//     }

//     pub fn op(lhs: &Self, rhs: &Self) -> Self {
//         assert_eq!(lhs.shape[1], rhs.shape[0]);
//         let h = lhs.shape[0];
//         let w = rhs.shape[1];
//         let n = lhs.shape[1];
//         let mut res = Self::new([h, w]);
//         for i in 0..h {
//             for j in 0..w {
//                 for k in 0..n {
//                     res[[i, j]] = res[[i, j]] + lhs[[i, k]]
// * rhs[[k, j]];                 } } } res }

//     pub fn pow(&self, n: usize) -> Self {
//         assert_eq!(self.shape[0], self.shape[1]);
//         if n == 0 {
//             return self.e();
//         }
//         let mut x = self.pow(n >> 1);
//         x = Self::op(&x, &x);
//         if n & 1 == 1 {
//             x = Self::op(&x, self);
//         }
//         x
//     }
// }

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
