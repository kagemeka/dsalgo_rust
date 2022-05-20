use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::new_rc_refcell::new_rc_refcell;

pub struct Node<T> {
    pub value: T,
    pub from: Vec<Weak<RefCell<Node<T>>>>,
    pub to: Vec<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value,
            from: Vec::new(),
            to: Vec::new(),
        }
    }

    pub fn new_rc_refcell(value: T) -> Rc<RefCell<Self>> {
        new_rc_refcell(Self::new(value))
    }

    pub fn connect(lhs: &Rc<RefCell<Self>>, rhs: &Rc<RefCell<Self>>) {
        lhs.borrow_mut().to.push(rhs.clone());
        rhs.borrow_mut().from.push(Rc::downgrade(lhs));
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            r#"Node {{
    value: {:?},
    from: [...] {} nodes,
    to: [...] {} nodes,
}}"#,
            self.value,
            self.from.len(),
            self.to.len(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug_print::debug_print;
    #[test]
    fn test() {
        #[derive(Debug)]
        pub struct Data {
            pub id: usize,
        }
        type V = Node<Data>;
        let a = V::new_rc_refcell(Data { id: 1 });
        let b = V::new_rc_refcell(Data { id: 2 });
        V::connect(&a, &b);
        debug_print(&a.borrow());
        debug_print(&b.borrow());

        V::connect(&b, &a);
        debug_print(&a.borrow());
        debug_print(&b.borrow());
    }
}
