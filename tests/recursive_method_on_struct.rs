struct Container {
    data: Vec<usize>,
}

impl Container {
    fn new(data: Vec<usize>) -> Self { Self { data } }

    fn add_sum(&mut self, i: usize) -> usize {
        self.data[i] += 1;
        if i == self.data.len() - 1 {
            return self.data[i];
        }
        self.data[i] + self.add_sum(i + 1)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let mut c = super::Container::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(c.add_sum(0), 55);
        assert_eq!(c.add_sum(5), 45);
        assert_eq!(c.add_sum(5), 50);
    }
}
