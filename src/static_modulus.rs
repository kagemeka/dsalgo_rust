use crate::modulus::Modulus;

/// new version, cannot compile on AtCoder yet.
/// ```
/// use dsalgo::{modular::Modular, static_modulus::StaticMod};
/// type Mint1_000_000_007 = Modular<StaticMod<1_000_000_007>>;
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StaticMod<const MOD: u32>;

impl<const MOD: u32> Modulus for StaticMod<MOD> {
    fn value() -> u32 { MOD }
}

// old version.
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
