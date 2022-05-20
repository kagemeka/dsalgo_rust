pub trait DefaultContext {}
impl<T> DefaultContext for T {}

/// why not std?;
/// because a `default` depends on the context.
pub trait Default<I: DefaultContext> {
    fn default() -> Self;
}
