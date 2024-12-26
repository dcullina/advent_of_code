use day_22::*;

const NUM_NEW_SECRET_NUMBERS: usize = 2000;

fn main() {
    let input: &str = include_str!("../input.txt");

    println!("Part One: {:?}", part_one::<NUM_NEW_SECRET_NUMBERS>(input));
    println!("Part Two: {:?}", part_two(input));
}
