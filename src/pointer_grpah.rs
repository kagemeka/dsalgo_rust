use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::new_rc_refcell::new_rc_refcell;

#[derive(Debug)]
pub struct Vertex<T, U> {
    pub value: T,
    pub edges: Vec<Edge<U, T>>,
}

impl<T: std::fmt::Debug, U> std::fmt::Display for Vertex<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Vertex {{ value: {:#?}, edges: [...] }}",
            self.value
        )
    }
}

impl<T, U> Vertex<T, U> {
    pub(crate) fn new(value: T) -> Self { Self { edges: Vec::new(), value } }

    pub fn new_rc_refcell(value: T) -> Rc<RefCell<Self>> {
        new_rc_refcell(Self::new(value))
    }

    pub fn connect(
        from: &Rc<RefCell<Self>>,
        to: &Rc<RefCell<Self>>,
        edge_data: U,
    ) {
        from.borrow_mut().edges.push(Edge::new(
            from,
            to.clone(),
            edge_data,
        ));
    }
}

#[derive(Debug)]
pub struct Edge<T, U> {
    pub value: T,
    pub from: Option<Weak<RefCell<Vertex<U, T>>>>,
    pub to: Rc<RefCell<Vertex<U, T>>>,
}

impl<T: std::fmt::Debug, U> std::fmt::Display for Edge<T, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Edge {{ from: ., to: ., value: {:#?} }}",
            self.value
        )
    }
}

impl<T, U> Edge<T, U> {
    pub(crate) fn new(
        from: &Rc<RefCell<Vertex<U, T>>>,
        to: Rc<RefCell<Vertex<U, T>>>,
        value: T,
    ) -> Self {
        Self {
            from: Some(Rc::downgrade(from)),
            to,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{debug_print::debug_print, new_rc_refcell::new_rc_refcell};

    #[test]
    fn test() {
        type V = Vertex<Option<usize>, Rc<RefCell<usize>>>;

        let a = V::new_rc_refcell(None);
        let b = V::new_rc_refcell(Some(1));
        debug_print(&a.borrow());

        V::connect(&a, &b, new_rc_refcell(1));
        debug_print(&a.borrow());
        debug_print(&b.borrow());
        *a.borrow().edges[0].value.borrow_mut() += 1;
        debug_print(&a.borrow());
    }
}
