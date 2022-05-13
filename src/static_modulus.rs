use crate::modulus::Modulus;

// new version, cannot compile on AtCoder yet.
// #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct StaticMod<const MOD: u32>;

// impl<const MOD: u32> Modulus for StaticMod<MOD> {
//     fn value() -> u32 { MOD }
// }

// type Mint1000000007 = Modular<StaticMod<1_000_000_007>>;
// type Mint998244353 = Modular<StaticMod<998_244_353>>;

// old version.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Mod1000000007;

impl Modulus for Mod1000000007 {
    fn value() -> u32 { 1_000_000_007 }
}
