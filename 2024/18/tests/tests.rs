
#[cfg(test)]
mod tests {
    use day_18::*;
    const GRID_DIM: usize = 6;
    const NUM_BYTES: usize = 12;

    #[test]
    fn test_part_one() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_one(test_input, GRID_DIM, NUM_BYTES), 22);
    }

    #[test]
    fn test_part_two() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_two(test_input, GRID_DIM, NUM_BYTES), "6,1");
    }

}
