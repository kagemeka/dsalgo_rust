use crate::choose::Choose;

pub struct HomogeneousProduct<T> {
    chooser: Box<dyn Choose<T>>,
}

impl<T> HomogeneousProduct<T> {
    pub fn new(chooser: Box<dyn Choose<T>>) -> Self { Self { chooser } }

    pub fn calc(&mut self, n: u64, k: u64) -> T
    where
        T: From<u64>,
    {
        if n == 0 {
            0.into()
        } else {
            self.chooser.choose(n + k - 1, k)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::HomogeneousProduct;
        use crate::{
            combination::Combination,
            modular_int::ModularInt,
            static_modulus::Mod1_000_000_007,
        };

        type Mint = ModularInt<Mod1_000_000_007>;
        let mut hom =
            HomogeneousProduct::<Mint>::new(Box::new(
                Combination::<Mint>::new(100),
            ));
        assert_eq!(hom.calc(5, 2).value(), 15);
    }
}
