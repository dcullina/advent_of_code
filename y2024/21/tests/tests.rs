#[cfg(test)]
mod tests {
    use day_21::*;

    #[test]
    fn test_part_one() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_one::<3>(test_input), 126384);
    }

    #[test]
    fn test_part_two() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_one::<26>(test_input), 0);
    }
}
