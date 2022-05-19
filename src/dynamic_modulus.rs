use crate::modulus::Modulus;

pub trait DynamicModId {}

impl<T> DynamicModId for T {}

/// ```
/// use dsalgo::dynamic_modulus::DynamicMod;
/// struct Foo;
/// type Mod = DynamicMod<Foo>;
/// Mod::set(1_000_000_007);
/// assert_eq!(Mod::value(), 1_000_000_007);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DynamicMod<Id: DynamicModId>(std::marker::PhantomData<Id>);

impl<I: DynamicModId> DynamicMod<I> {
    fn core() -> &'static std::sync::atomic::AtomicU32 {
        // cannot return &'static mut VALUE because it can be changed by
        // multiple threads.
        // so we should return a reference of a type having interior mutability.
        // cannot use std::cell for static value because it is not
        // `Sync`;
        // we should use std::sync types instead.
        // atomic types are lighter than mutex.

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
    fn value() -> u32 { Self::core().load(std::sync::atomic::Ordering::SeqCst) }
}
