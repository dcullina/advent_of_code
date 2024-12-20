
#[cfg(test)]
mod tests {
    use day_11::*;

    const N_BLINKS: usize = 25;

    #[test]
    fn test_part_one() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_one_two(test_input, N_BLINKS), 55312);
    }
}
