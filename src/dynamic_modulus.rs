use crate::modulus::Modulus;

pub trait DynamicModId {}

impl<T> DynamicModId for T {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DynamicMod<Id: DynamicModId>(std::marker::PhantomData<Id>);

impl<I: DynamicModId> DynamicMod<I> {
    fn core() -> &'static std::sync::atomic::AtomicU32 {
        // VALUE type needs Sync + 'static
        // std::cell types are not Sync
        // std::sync::Mutex is not 'static
        // only atomic types can be.
        // or we can use external crate like `lazy_static`.

        // why not defining as associated const variable?
        // -> const variables are immutabe in any situation.

        static VALUE: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);
        &VALUE
    }

    pub fn set(value: u32) {
        assert!(value > 1);
        Self::core().store(
            value,
            std::sync::atomic::Ordering::SeqCst,
        );
    }
}

impl<I: DynamicModId> Modulus for DynamicMod<I> {
    fn modulus() -> u32 {
        Self::core().load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        struct Foo;
        type Mod = DynamicMod<Foo>;
        Mod::set(1_000_000_007);
        assert_eq!(Mod::modulus(), 1_000_000_007);
    }
}
