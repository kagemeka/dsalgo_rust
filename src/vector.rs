#[derive(Debug, Clone)]
pub struct Vector<T, const DIM: usize> {
    data: [T; DIM],
}

impl<T: Default + Copy, const DIM: usize> Default for Vector<T, DIM> {
    fn default() -> Self { Self { data: [T::default(); DIM] } }
}

impl<T: Default + Copy + std::ops::Add<Output = T>, const DIM: usize>
    std::ops::Add<Self> for Vector<T, DIM>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut res = Self::default();
        for i in 0..DIM {
            res.data[i] = self.data[i] + rhs.data[i];
        }
        res
    }
}

impl<T: Default + Copy + std::ops::Sub<Output = T>, const DIM: usize>
    std::ops::Sub<Self> for Vector<T, DIM>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut res = Self::default();
        for i in 0..DIM {
            res.data[i] = self.data[i] - rhs.data[i];
        }
        res
    }
}

impl<T: Default + Copy + std::ops::Neg<Output = T>, const DIM: usize>
    std::ops::Neg for Vector<T, DIM>
{
    type Output = Self;

    fn neg(self) -> Self {
        let mut res = Self::default();
        for i in 0..DIM {
            res.data[i] = -self.data[i];
        }
        res
    }
}

impl<T, const DIM: usize> Vector<T, DIM>
where
    T: Copy + std::ops::Add<Output = T> + std::ops::Mul<Output = T>,
{
    pub fn inner_prod(&self, rhs: &Self) -> T {
        self.data
            .iter()
            .zip(rhs.data)
            .map(|(&x, y)| x * y)
            .reduce(T::add)
            .unwrap()
    }
}

impl<T: Copy + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>>
    Vector<T, 2>
{
    pub fn cross_prod(&self, rhs: &Self) -> T {
        self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0]
    }
}

#[derive(Clone)]
pub struct LineSegment<T: Clone, const DIM: usize> {
    v0: Vector<T, DIM>,
    v1: Vector<T, DIM>,
}

impl LineSegment<i64, 2> {
    pub fn across(&self, rhs: &Self) -> bool {
        let v0 = rhs.v1.clone() - rhs.v0.clone();
        let v1 = self.v0.clone() - rhs.v0.clone();
        let v2 = self.v1.clone() - rhs.v0.clone();
        v0.cross_prod(&v1) * v0.cross_prod(&v2) <= 0
    }

    pub fn intersect(lhs: &Self, rhs: &Self) -> bool {
        lhs.across(rhs) && rhs.across(lhs)
    }
}
