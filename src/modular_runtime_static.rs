use crate::modular_static::Modulus;
pub struct RuntimeModulusCore {
    value: std::sync::atomic::AtomicUsize,
}

impl RuntimeModulusCore {
    pub const fn new(value: usize) -> Self {
        Self {
            value: std::sync::atomic::AtomicUsize::new(value),
        }
    }

    pub fn get_value(&self) -> usize { self.value.load(std::sync::atomic::Ordering::SeqCst) }

    pub fn set_value(&self, value: usize) {
        assert!(value > 1);
        self.value.store(value, std::sync::atomic::Ordering::SeqCst);
    }
}

/// # Examples
/// ```
/// use dsalgo::{modular_runtime_static::*, modular_static::*};
/// struct Mod100;
/// impl RuntimeModulus for Mod100 {
///     fn core() -> &'static RuntimeModulusCore {
///         static MODULUS: RuntimeModulusCore = RuntimeModulusCore::new(1);
///         &MODULUS
///     }
/// }
/// impl Modulus for Mod100 {
///     fn value() -> usize { Self::core().get_value() }
/// }
/// Mod100::set(100);
/// assert_eq!(Mod100::value(), 100);
/// ```
pub trait RuntimeModulus: Modulus {
    fn core() -> &'static RuntimeModulusCore;
    fn set(value: usize) { Self::core().set_value(value); }
}

#[cfg(test)]
mod tests {
    use super::{Modulus as _, RuntimeModulus as _};
    use crate::modular_static::Modular;
    #[test]
    fn test() {
        fn define_runtime(n: usize) {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            struct ModRuntime;

            impl super::RuntimeModulus for ModRuntime {
                fn core() -> &'static super::RuntimeModulusCore {
                    static MODULUS: super::RuntimeModulusCore = super::RuntimeModulusCore::new(1);
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
