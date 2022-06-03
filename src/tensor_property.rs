pub trait Shape {
    fn shape() -> &'static [usize];
}

pub trait Size {
    fn size() -> usize;
}

impl<T: Shape> Size for T {
    fn size() -> usize {
        let mut size = 1;
        for &dim in Self::shape() {
            size *= dim;
        }
        size
    }
}

pub trait Dimension {
    fn dimension() -> usize;
}

impl<P: Shape> Dimension for P {
    fn dimension() -> usize { P::shape().len() }
}

pub trait Strides {
    fn strides() -> Vec<usize>;
}

impl<P: Shape> Strides for P {
    fn strides() -> Vec<usize> {
        let ndim = Self::dimension();
        let mut strides: Vec<usize> = P::shape().into();
        if ndim > 0 {
            strides[ndim - 1] = 1;
        }
        for i in (1..ndim).rev() {
            strides[i - 1] = strides[i] * P::shape()[i];
        }
        strides
    }
}

pub trait TensorProperty: Shape {}
impl<P: Shape> TensorProperty for P {}
