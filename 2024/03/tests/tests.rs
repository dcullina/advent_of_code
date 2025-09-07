#[cfg(test)]
mod tests {
    use day_3::*;

    #[test]
    fn test_part_one() {
        let test_input: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part_one(test_input), 161)
    }

    #[test]
    fn test_part_two() {
        let test_input: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part_two(test_input), 48)
    }
}