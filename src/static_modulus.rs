use crate::modulus::Modulus;

/// new version, cannot compile on AtCoder yet.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StaticMod<const MOD: u32>;

impl<const MOD: u32> Modulus for StaticMod<MOD> {
    fn value() -> u32 { MOD }
}

/// old version.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mod1_000_000_007;

impl Modulus for Mod1_000_000_007 {
    fn value() -> u32 { 1_000_000_007 }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mod998_244_353;

impl Modulus for Mod998_244_353 {
    fn value() -> u32 { 998_244_353 }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::StaticMod;
        use crate::modular::Modular;
        type Mint = Modular<StaticMod<1_000_000_007>>;
        let a = Mint::from(1_000_000_008);
        assert_eq!(a.value(), 1);
    }
}
