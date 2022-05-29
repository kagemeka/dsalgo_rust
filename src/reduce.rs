pub fn reduce<T, F, I>(f: F, values: I) -> Option<T>
where
    F: Fn(T, T) -> T,
    I: Iterator<Item = T>,
{
    values.reduce(f)
}

// TODO
#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
