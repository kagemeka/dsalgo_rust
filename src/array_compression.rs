pub struct CompressionResult<T> {
    pub keys: Vec<usize>,
    pub values: Vec<T>,
}

pub fn compress<T: Ord + Clone>(slice: &[T]) -> CompressionResult<T> {
    let values = crate::vector_util::unique(slice);
    let keys = slice
        .iter()
        .map(|x| values.binary_search(x).unwrap())
        .collect::<Vec<_>>();
    CompressionResult { keys, values }
}

pub struct ArrayCompression<T: Ord + Clone> {
    values: Vec<T>,
}

impl<T: Ord + Clone> ArrayCompression<T> {
    pub fn new(slice: &[T]) -> Self {
        Self {
            values: crate::vector_util::unique(slice),
        }
    }

    pub fn encode(&self, value: &T) -> Option<usize> {
        if let Ok(key) = self.values.binary_search(value) {
            Some(key)
        } else {
            None
        }
    }

    pub fn decode(&self, key: usize) -> T { self.values[key].clone() }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let arr = [4, 3, 0, -1, 3, 10];
        let compression = super::ArrayCompression::new(&arr);
        assert_eq!(compression.encode(&-1).unwrap(), 0);
        assert_eq!(compression.encode(&10).unwrap(), 4);
        assert_eq!(compression.decode(0), -1);
        assert_eq!(compression.encode(&5), None);

        let result = super::compress(&arr);
        assert_eq!(result.keys, vec![3, 2, 1, 0, 2, 4]);
        assert_eq!(result.values, vec![-1, 0, 3, 4, 10]);
    }
}
