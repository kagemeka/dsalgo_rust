#[allow(dead_code)]
pub(crate) fn debug_print<T: std::fmt::Debug>(data: &T) {
    eprintln!("{:#?} ", data);
}
