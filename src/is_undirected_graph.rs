// TODO: check whether values are pointing to the same address or not.
// we need to consider corner cases like
// - Rc<Refcell(NotSync)<T>>
// - Arc<Mutex(Sync)<T>>
