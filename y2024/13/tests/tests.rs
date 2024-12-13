
#[cfg(test)]
mod tests {
    use day_13::*;

    #[test]
    fn test_part_one() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_one(test_input), 480);
    }
}
