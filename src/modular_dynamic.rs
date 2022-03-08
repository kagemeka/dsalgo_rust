use crate::modular::Modulus;
pub struct DynamicModulusCore {
    value: std::sync::atomic::AtomicUsize,
}

impl DynamicModulusCore {
    pub const fn new(value: usize) -> Self {
        Self {
            value: std::sync::atomic::AtomicUsize::new(value),
        }
    }

    pub fn get_value(&self) -> usize { self.value.load(std::sync::atomic::Ordering::SeqCst) }

    pub fn set_value(&self, value: usize) {
        self.value.store(value, std::sync::atomic::Ordering::SeqCst);
    }
}

/// # Examples
/// ```
/// use dsalgo::{modular::*, modular_dynamic::*};
/// struct Mod100;
/// impl DynamicModulus for Mod100 {
///     fn core() -> &'static DynamicModulusCore {
///         static MODULUS: DynamicModulusCore = DynamicModulusCore::new(1);
///         &MODULUS
///     }
/// }
/// impl Modulus for Mod100 {
///     fn value() -> usize { Self::core().get_value() }
/// }
/// Mod100::set(100);
/// assert_eq!(Mod100::value(), 100);
/// ```
pub trait DynamicModulus: Modulus {
    fn core() -> &'static DynamicModulusCore;
    fn set(value: usize) { Self::core().set_value(value); }
}

#[cfg(test)]
mod tests {
    use super::{DynamicModulus as _, Modulus as _};
    use crate::modular::Modular;
    #[test]
    fn test() {
        fn define_runtime(n: usize) {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            struct ModRuntime;

            impl super::DynamicModulus for ModRuntime {
                fn core() -> &'static super::DynamicModulusCore {
                    static MODULUS: super::DynamicModulusCore = super::DynamicModulusCore::new(1);
                    &MODULUS
                }
            }

            impl super::Modulus for ModRuntime {
                fn value() -> usize { Self::core().get_value() }
            }

            ModRuntime::set(n);
            assert_eq!(ModRuntime::value(), n);
            type Mint = Modular<ModRuntime>;
            let x = Mint::new(5);
            println!("{}", x);
            assert_eq!(x * x, Mint::new(25));
        }

        define_runtime(100);
    }
}
