use std::{cell::RefCell, rc::Rc};

pub(crate) fn new_rc_refcell<T>(value: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(value))
}
