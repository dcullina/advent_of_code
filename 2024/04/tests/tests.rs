#[cfg(test)]
mod tests {
    use day_4::*;

    #[test]
    fn test_part_one() {
        let test_input: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n";
        assert_eq!(part_one(test_input), 18);
    }

    #[test]
    fn test_part_two() {
        let test_input: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX\n";
        assert_eq!(part_two(test_input), 9);
    }
}
