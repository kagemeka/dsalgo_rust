/// Tensor NDIM >= 1
#[derive(Clone)]
pub struct Tensor<T, const NDIM: usize> {
    shape: [usize; NDIM],
    strides: [usize; NDIM],
    pub size: usize,
    data: Vec<T>,
}

impl<T: std::fmt::Debug, const NDIM: usize> std::fmt::Debug for Tensor<T, NDIM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tensor")
            .field("shape", &self.shape)
            .field("data", &self.data)
            .finish()
    }
}

impl<T: Clone + Default, const NDIM: usize> Tensor<T, NDIM> {
    pub fn new(shape: [usize; NDIM]) -> Self {
        let shape = shape.clone();
        let mut strides: [usize; NDIM] = shape;
        strides[NDIM - 1] = 1;
        for i in (1..NDIM).rev() {
            strides[i - 1] = strides[i] * shape[i];
        }
        let size: usize = strides[0] * shape[0];
        let data = vec![T::default(); size];
        Self {
            shape,
            strides,
            size,
            data,
        }
    }
}

impl<T, const NDIM: usize> Tensor<T, NDIM> {
    fn flat_index(&self, index: [usize; NDIM]) -> usize {
        let mut idx = 0;
        for i in 0..NDIM {
            idx += index[i] * self.strides[i];
        }
        idx
    }
}

/// https://doc.rust-lang.org/std/ops/trait.Index.html
impl<T, const NDIM: usize> std::ops::Index<[usize; NDIM]> for Tensor<T, NDIM> {
    type Output = T;

    fn index(&self, index: [usize; NDIM]) -> &Self::Output { &self.data[self.flat_index(index)] }
}

/// https://doc.rust-lang.org/std/ops/trait.IndexMut.html
impl<T, const NDIM: usize> std::ops::IndexMut<[usize; NDIM]> for Tensor<T, NDIM> {
    fn index_mut(&mut self, index: [usize; NDIM]) -> &mut Self::Output {
        let idx = self.flat_index(index);
        &mut self.data[idx]
    }
}

impl<T> std::ops::Mul for Tensor<T, 2>
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
