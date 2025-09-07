#[cfg(test)]
mod tests {
    use day_20::*;

    const MIN_GAIN_P1: usize = 40;
    const CHEAT_LENGTH_P1: usize = 2;

    #[test]
    fn test_part_one() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_one_two::<CHEAT_LENGTH_P1, MIN_GAIN_P1>(test_input), 2);
    }

    const MIN_GAIN_P2: usize = 50;
    const CHEAT_LENGTH_P2: usize = 20;

    #[test]
    fn test_part_two() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(
            part_one_two::<CHEAT_LENGTH_P2, MIN_GAIN_P2>(test_input),
            285
        );
    }
}
