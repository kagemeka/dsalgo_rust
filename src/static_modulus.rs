use crate::modulus::Modulus;

/// new version, cannot compile on AtCoder yet.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StaticMod<const MOD: u32>;

impl<const MOD: u32> Modulus for StaticMod<MOD> {
    fn modulus() -> u32 { MOD }
}

// old version.
macro_rules! define_static_modulus {
    ($typename:ident, $value:expr) => {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $typename;

        impl Modulus for $typename {
            fn modulus() -> u32 { $value }
        }
    };
}

define_static_modulus!(Mod998_244_353, 998_244_353);
define_static_modulus!(Mod1_000_000_007, 1_000_000_007);

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::StaticMod;
        use crate::modular_int::ModularInt;
        type Mint = ModularInt<StaticMod<1_000_000_007>>;
        let a = Mint::from(1_000_000_008);
        assert_eq!(a.value(), 1);
    }
}
