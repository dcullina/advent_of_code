
#[cfg(test)]
mod tests {
    use day_14::*;

    const NUM_X_TILES: usize = 11;
    const NUM_Y_TILES: usize = 7;

    #[test]
    fn test_part_one() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_one(test_input, NUM_X_TILES, NUM_Y_TILES), 12);
    }
}
