pub trait Search<T> {
    type SearchFn = fn(&T) -> bool;
    fn search(&self, is_ok: SearchFn) -> Option<usize>;
}
