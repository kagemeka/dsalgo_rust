#[cfg(test)]
mod tests {
    #[test]
    fn test_next_power_of_two() {
        assert_eq!(0usize.next_power_of_two(), 1);
        assert_eq!(1usize.next_power_of_two(), 1);
        assert_eq!(2usize.next_power_of_two(), 2);
        assert_eq!(3usize.next_power_of_two(), 4);
    }

    #[test]
    fn negative_division() {
        assert_eq!(-5 % 2, -1);
        assert_eq!(-5 % -2, -1);
        assert_eq!(5 % -2, 1);
    }
}
