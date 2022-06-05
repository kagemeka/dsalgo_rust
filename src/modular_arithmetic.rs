use crate::{modular_inverse_extgcd::modular_inverse_extgcd, modulus::Modulus};

pub struct ModularArithemetic<M>(std::marker::PhantomData<M>);

impl<M: Modulus> ModularArithemetic<M> {
    pub fn add(mut x: u32, rhs: u32) -> u32 {
        x += rhs;
        if x >= M::modulus() {
            x -= M::modulus();
        }
        x
    }

    pub fn neg(x: u32) -> u32 { if x == 0 { 0 } else { M::modulus() - x } }

    pub fn sub(x: u32, rhs: u32) -> u32 { Self::add(x, Self::neg(rhs)) }

    pub fn mul(x: u32, rhs: u32) -> u32 {
        (x as u64 * rhs as u64 % M::modulus() as u64) as u32
    }

    /// unlike extgcd, the caller cannot eunsure the inverse exist.
    /// with additional constant run time cost before calling this function.
    /// so if the inverse element does not exit,
    /// handle execption inside the method, and return Result<T, E>
    pub fn invert(x: u32) -> Result<u32, &'static str> {
        assert!(x > 0);
        Ok(modular_inverse_extgcd(M::modulus() as u64, x as u64)? as u32)
    }

    pub fn div(x: u32, rhs: u32) -> u32 {
        Self::mul(x, Self::invert(rhs).unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
