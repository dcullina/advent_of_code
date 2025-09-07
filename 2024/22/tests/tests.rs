#[cfg(test)]
mod tests {
    use day_22::*;

    const NUM_NEW_SECRET_NUMBERS: usize = 2000;

    #[test]
    fn test_part_one() {
        let test_input: &str = include_str!("../test_input_1.txt");
        assert_eq!(part_one::<NUM_NEW_SECRET_NUMBERS>(test_input), 37327623);
    }

    #[test]
    fn test_part_two() {
        let test_input: &str = include_str!("../test_input_2.txt");
        assert_eq!(part_two::<NUM_NEW_SECRET_NUMBERS>(test_input), 23);
    }
}
