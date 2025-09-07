#[cfg(test)]
mod tests {
    use day_24::*;

    #[test]
    fn test_part_one() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(part_one(test_input), 2024);
    }

    #[test]
    fn test_part_two() {
        let test_input: &str = include_str!("../test_input.txt");
        assert_eq!(
            part_two(test_input),
            "aaa,aoc,bbb,ccc,eee,ooo,z24,z99".to_string()
        );
    }
}
