
#[cfg(test)]
mod tests {
    use day_16::*;

    #[test]
    fn test_part_one_1() {
        let test_input1: &str = include_str!("../test_input1.txt");
        assert_eq!(part_one(test_input1), 7036);
    }

    #[test]
    fn test_part_one_2() {
        let test_input2: &str = include_str!("../test_input2.txt");
        assert_eq!(part_one(test_input2), 11048);
    }

    // #[test]
    // fn test_part_two() {
    //     let test_input: &str = include_str!("../test_input.txt");
    //     assert_eq!(part_two(test_input1), 0);
    // }

}
