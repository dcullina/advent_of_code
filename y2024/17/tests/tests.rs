
#[cfg(test)]
mod tests {
    use day_17::*;

    #[test]
    fn test_part_one() {
        let test_input: &str = include_str!("../test_input1.txt");
        assert_eq!(part_one(test_input), "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn test_part_two() {
        let test_input: &str = include_str!("../test_input2.txt");
        assert_eq!(part_two(test_input), 117440);
    }

}
