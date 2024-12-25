use day_21::*;

fn main() {
    let input: &str = include_str!("../input.txt");

    println!("Part One: {:?}", part_one::<3>(input));
    println!("Part Two: {:?}", part_one::<26>(input));
}
