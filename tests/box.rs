#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        #[derive(Debug)]
        struct Data {
            value: usize,
        }
        let a = Data { value: 1 };
        let mut b = Box::new(a);
        b.value += 1;
        println!("{:?}", b);
        // let mut c = Box::new(a);
        // compile error because a has been removed.
    }
}
