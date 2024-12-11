use day_11::*;

const N_BLINKS: usize = 25;

fn main() {
    let input: &str = include_str!("../input.txt");

    println!("Part One: {:?}", part_one(input, N_BLINKS));
    println!("Part Two: {:?}", part_two(input, N_BLINKS));
}
