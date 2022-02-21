use super::{abstract_::structure::traits::*, pow::pow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Modular<const MOD: usize> {
    value: usize,
}

impl<const MOD: usize> std::fmt::Display for Modular<MOD> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<const MOD: usize> Modular<MOD> {
    pub fn new(n: usize) -> Self { Self { value: n % MOD } }

    pub const fn value(&self) -> usize { self.value % MOD }

    pub fn inverse(&self) -> Modular<MOD> { pow(self, MOD - 2) }
}

impl<const MOD: usize> MulIdentity for Modular<MOD> {
    fn e() -> Self { Self { value: 1 } }
}

impl<const MOD: usize> Identity for Modular<MOD> {
    fn e() -> Self { Self { value: 1 } }
}

impl<const MOD: usize> Semigroup for Modular<MOD> {
    const COMMUTATIVE: bool = true;
    const IDEMPOTENT: bool = false;

    fn op(x: &Self, y: &Self) -> Self { *x * *y }
}

impl<const MOD: usize> Monoid for Modular<MOD> {}

impl<const MOD: usize> std::ops::AddAssign for Modular<MOD> {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl<const MOD: usize> std::ops::Add for Modular<MOD> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            value: (self.value + rhs.value) % MOD,
        }
    }
}

impl<const MOD: usize> std::ops::Neg for Modular<MOD> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            value: MOD - self.value,
        }
    }
}

impl<const MOD: usize> std::ops::SubAssign for Modular<MOD> {
    fn sub_assign(&mut self, rhs: Self) { *self += -rhs; }
}

impl<const MOD: usize> std::ops::Sub for Modular<MOD> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self { self + -rhs }
}

impl<const MOD: usize> std::ops::Mul for Modular<MOD> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            value: self.value * rhs.value % MOD,
        }
    }
}

impl<const MOD: usize> std::ops::MulAssign for Modular<MOD> {
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

impl<const MOD: usize> std::ops::Div for Modular<MOD> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self { self * rhs.inverse() }
}

impl<const MOD: usize> std::ops::DivAssign for Modular<MOD> {
    fn div_assign(&mut self, rhs: Self) { *self = *self / rhs; }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Mint = Modular<1_000_000_007>;

    #[test]
    fn test_new() {
        assert_eq!(Mint::new(1_000_000_008), Mint::new(1));
    }

    #[test]
    fn test_add() {
        let a = Mint::new(1);
        let b = Mint::new(1_000_000_007);
        assert_eq!(a + b, Mint::new(1));
        assert_eq!(b, Mint::new(0));
    }

    #[test]
    fn test_add_assign() {
        let mut a = Mint::new(1);
        let b = Mint::new(1_000_000_007);
        a += b;
        assert_eq!(a, Mint::new(1));
        assert_eq!(b, Mint::new(0));
    }

    #[test]
    fn test_neg() {
        let a = Mint::new(1);
        assert_eq!(-a, Mint::new(1_000_000_006));
    }

    #[test]
    fn test_sub() {
        let a = Mint::new(1);
        let b = Mint::new(1_000_000_007);
        assert_eq!(a - b, Mint::new(1));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Mint::new(1);
        let b = Mint::new(1_000_000_007);
        a -= b;
        assert_eq!(a, Mint::new(1));
    }

    #[test]
    fn test_mul() {
        let a = Mint::new(2);
        let b = Mint::new(1_000_000_006);
        assert_eq!(a * b, Mint::new(1_000_000_005));
        assert_eq!(b, Mint::new(1_000_000_006));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = Mint::new(2);
        let b = Mint::new(1_000_000_006);
        a *= b;
        assert_eq!(a, Mint::new(1_000_000_005));
        assert_eq!(b, Mint::new(1_000_000_006));
    }

    #[test]
    fn test_inverse() {
        let a = Mint::new(2);
        assert_eq!(a.inverse(), Mint::new(500000004));
    }
}
