pub trait StaticShape {
    fn shape() -> crate::matrix::Shape;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
