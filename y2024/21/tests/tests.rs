#[cfg(test)]
mod tests {
    use day_21::*;

    const NUM_ROBOTS_P1: usize = 3;

    #[test]
    fn test_part_one() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_one_two::<NUM_ROBOTS_P1>(test_input), 126384);
    }
}
