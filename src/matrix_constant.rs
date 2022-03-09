pub trait ConstantShape {
    const HEIGHT: usize;
    const WIDTH: usize;
    type Data;
}

// pub struct Matrix4x4<T, S: ConstantShape> {
//     data: S::Data,
// }
