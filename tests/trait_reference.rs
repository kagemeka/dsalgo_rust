#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        trait Get<'a> {
            type Output: Constraints;
            fn get(self) -> Self::Output;
        }

        trait Constraints {}
        impl<'b> Constraints for &'b mut usize {}

        impl<'a> Get<'a> for &'a mut usize {
            type Output = &'a mut usize;

            fn get(self) -> Self::Output { self }
        }
    }
}
