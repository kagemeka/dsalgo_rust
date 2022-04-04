#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        #[derive(Debug)]
        struct Data {
            value: usize,
        }

        impl Data {
            fn mut_value(&mut self) -> &mut usize { &mut self.value }
        }

        let mut data = Data { value: 0 };
        *data.mut_value() += 1;
        println!("{:?}", data);
    }
}
