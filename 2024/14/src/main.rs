use day_14::*;

const NUM_X_TILES: usize = 101;
const NUM_Y_TILES: usize = 103;

fn main() {
    let input: &str = include_str!("../input.txt");

    println!("Part One: {:?}", part_one(input, NUM_X_TILES, NUM_Y_TILES));
    println!("Part Two:");
    part_two(input, NUM_X_TILES, NUM_Y_TILES);
}
