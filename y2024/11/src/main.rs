use day_11::*;

const N_BLINKS_P1: usize = 25;
const N_BLINKS_P2: usize = 75;

fn main() {
    let input: &str = include_str!("../input.txt");

    println!("Part One: {:?}", part_one_two(input, N_BLINKS_P1));
    println!("Part Two: {:?}", part_one_two(input, N_BLINKS_P2));
}
