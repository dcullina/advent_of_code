use day_21::*;

const NUM_ROBOTS_P1: usize = 3;
const NUM_ROBOTS_P2: usize = 26;

fn main() {
    let input: &str = include_str!("../input.txt");

    println!("Part One: {:?}", part_one_two::<NUM_ROBOTS_P1>(input));
    println!("Part Two: {:?}", part_one_two::<NUM_ROBOTS_P2>(input));
}
