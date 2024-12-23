#[cfg(test)]
mod tests {
    use day_21::*;

    #[test]
    fn test_part_one() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_one(test_input), 0);
    }

    #[test]
    fn test_part_two() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_two(test_input), 0);
    }
}
