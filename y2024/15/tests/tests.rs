
#[cfg(test)]
mod tests {
    use day_15::*;

    #[test]
    fn test_part_one_1() {
        let test_input1: &str = include_str!("../test_input1.txt");
        assert_eq!(part_one(test_input1), 10092);
    }

    #[test]
    fn test_part_one_2() {
        let test_input2: &str = include_str!("../test_input2.txt");
        assert_eq!(part_one(test_input2), 2028);
    }

    #[test]
    fn test_part_two() {
        let test_input1: &str = include_str!("../test_input1.txt");
        assert_eq!(part_two(test_input1), 9021);
    }

}
