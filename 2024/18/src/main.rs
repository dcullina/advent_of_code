use day_18::*;

const GRID_DIM: usize = 70;
const NUM_BYTES: usize = 1024;


fn main() {
    let input: &str = include_str!("../input.txt");

    println!("Part One: {:?}", part_one(input, GRID_DIM, NUM_BYTES));
    println!("Part Two: {:?}", part_two(input, GRID_DIM, NUM_BYTES));
}
