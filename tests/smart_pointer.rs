#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use std::{cell, rc};

        let a = rc::Rc::new(cell::RefCell::new(1));
        let p1 = &a as *const _;
        let p2 = &a.borrow() as *const _;
        let mut p3 = &a.borrow() as *const _;
        println!("{:?}", p1);
        println!("{:?}", p2);
        println!("{:?}", p3);
        p3 = &a.borrow() as *const _;
        println!("{:?}", p3);
        println!("{:?}", a.as_ptr());
        println!(
            "{:?}",
            &*a.borrow() as *const _
        );
        assert!(std::ptr::eq(
            a.as_ptr(),
            &*a.borrow()
        ));

        let a = cell::RefCell::new(1);
        let p1 = &a as *const _;
        let p2 = &a.borrow() as *const _;
        let mut p3 = &a.borrow() as *const _;
        println!("{:?}", p1);
        println!("{:?}", p2);
        println!("{:?}", p3);
        p3 = &a.borrow() as *const _;
        println!("{:?}", p3);
        let p4 = &a as *const _;
        println!("{:?}", p4);
        let tmp = a.as_ptr();
        let p5 = &*a.borrow() as *const _;
        println!("{:?}", p5);
        println!("{:?}", tmp);
        let tmp = a.borrow();
        println!("{:?}", &*tmp as *const _);
    }
}
