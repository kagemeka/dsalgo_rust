pub trait MulInv {
    type Output;
    fn mul_inv(self) -> Self::Output;
}
