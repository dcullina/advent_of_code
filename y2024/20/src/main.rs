use day_20::*;

const MIN_GAIN: usize = 100;
const CHEAT_LENGTH_P1: usize = 2;
const CHEAT_LENGTH_P2: usize = 20;

fn main() {
    let input: &str = include_str!("../input.txt");

    println!(
        "Part One: {:?}",
        part_one_two::<CHEAT_LENGTH_P1, MIN_GAIN>(input)
    );
    println!(
        "Part Two: {:?}",
        part_one_two::<CHEAT_LENGTH_P2, MIN_GAIN>(input)
    );
}
