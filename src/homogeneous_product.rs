use crate::choose::Choose;

/// Example
/// ```
/// use dsalgo::{
///     combination::Combination,
///     homogeneous_product::HomogeneousProduct,
///     modular::Modular,
///     static_modulus::Mod1_000_000_007,
/// };
/// type Mint = Modular<Mod1_000_000_007>;
/// let hom = HomogeneousProduct::<Mint>::new(Box::new(
///     Combination::<Mint>::new(100),
/// ));
/// assert_eq!(hom.calc(5, 2).value(), 15);
/// ```
pub struct HomogeneousProduct<T>
where
    T: From<usize>,
{
    chooser: Box<dyn Choose<T>>,
}

impl<T> HomogeneousProduct<T>
where
    T: From<usize>,
{
    pub fn new(chooser: Box<dyn Choose<T>>) -> Self { Self { chooser } }

    pub fn calc(&self, n: usize, k: usize) -> T {
        if n == 0 {
            0.into()
        } else {
            self.chooser.choose(n + k - 1, k)
        }
    }
}
