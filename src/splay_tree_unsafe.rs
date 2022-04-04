pub struct Node<S: Default> {
    left: *mut Node<S>,
    right: *mut Node<S>,
    parent: *mut Node<S>,
    size: usize,
    value: S,
}

impl<S: Default> Default for Node<S> {
    fn default() -> Self {
        Self {
            left: std::ptr::null_mut(),
            right: std::ptr::null_mut(),
            parent: std::ptr::null_mut(),
            size: 1,
            value: S::default(),
        }
    }
}

pub struct SplayTree<S: Default> {
    root: *mut Node<S>,
}

impl<S: Default> SplayTree<S> {
    unsafe fn update(u: &mut Node<S>) {
        u.size = 1;
        if let Some(v) = u.left.as_mut() {
            u.size += v.size;
        }
        if let Some(v) = u.right.as_mut() {
            u.size += v.size;
        }
    }

    unsafe fn rotate(u: &mut Node<S>) {
        let p = u.parent.as_mut().unwrap();
        let pp = p.parent;
        let c: *mut Node<S>;
        if p.left == u {
            c = u.right;
            u.right = p;
            p.left = c;
        } else {
            c = u.left;
            u.left = p;
            p.right = c;
        }
        p.parent = u;
        if let Some(c) = c.as_mut() {
            c.parent = p;
        }
        if let Some(pp) = pp.as_mut() {
            if std::ptr::eq(pp.left, p) {
                pp.left = u;
            } else {
                pp.right = u;
            }
        }
        u.parent = pp;
        Self::update(p);
        Self::update(u);
    }

    unsafe fn splay(u: &mut Node<S>) {
        while Self::state(u) != 0 {
            let p = u.parent.as_mut().unwrap();
            if Self::state(p) == 0 {
                Self::rotate(u);
            } else if Self::state(p) == Self::state(u) {
                Self::rotate(p);
                Self::rotate(u);
            } else {
                Self::rotate(u);
                Self::rotate(p);
            }
        }
    }

    unsafe fn state(u: &Node<S>) -> i8 {
        if let Some(p) = u.parent.as_ref() {
            if std::ptr::eq(p.left, u) { 1 } else { -1 }
        } else {
            0
        }
    }

    // pub unsafe fn get(&self, i: usize) -> &'static mut Node<S> {
    //     let mut u = self.root;
    //     loop {
    //         let lsize = if let Some(v) = u.left.as_ref() {
    // v.size } else { 0 };         if i < lsize {
    //             u = u.left.as_ref().unwrap();
    //         } else if i == lsize {
    //             Self::splay(u);
    //         }
    //     }
    // }
}
