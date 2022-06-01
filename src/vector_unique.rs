pub fn vector_unique<T: Ord>(mut values: Vec<T>) -> Vec<T> {
    values.sort();
    values.dedup();
    values
}

// TODO:
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
