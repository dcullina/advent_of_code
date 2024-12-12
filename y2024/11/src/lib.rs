use std::time::Instant;
use rayon::prelude::*;

const MUL_YEAR: usize = 2024;

pub fn part_one_two(input: &str, n_blinks: usize) -> usize {
    let mut stones: Vec<usize> = Vec::new();
    input
        .split_ascii_whitespace()
        .for_each(|number| {
            stones.push(number.parse::<usize>().unwrap());
        });

    for i in 0..n_blinks {
        let start = Instant::now();
        let mut new_stones: Vec<usize> = Vec::new();
        let mut index: usize = 0;

        while index < stones.len() {
            if stones[index] == 0 {
                stones[index] = 1;
            } else {
                let mut stone_value: usize = stones[index];
                let mut stone_value_digits: Vec<usize> = Vec::new();

                while stone_value > 0 {
                    stone_value_digits.push(stone_value % 10);
                    stone_value /= 10;
                }
                stone_value_digits.reverse();

                if stone_value_digits.len() % 2 == 0 {
                    let mid = stone_value_digits.len() / 2;
                    let left = stone_value_digits[..mid].iter().fold(0, |acc, &d| acc * 10 + d);
                    let right = stone_value_digits[mid..].iter().fold(0, |acc, &d| acc * 10 + d);

                    stones[index] = left;
                    new_stones.push(right);
                } else {
                    stones[index] *= MUL_YEAR;
                }

                /*if stone_value.chars().count() % 2 == 0 {

                    let (left, right) = val_str.split_at(val_str.len() / 2);

                    stones[index] = left.parse::<usize>().unwrap();

                    if stones.len() == index + 1 {
                        stones.push(right.parse::<usize>().unwrap());
                    } else {
                        stones.insert(index + 1, right.parse::<usize>().unwrap())
                    }

                    index += 1;
                } else {
                    stones[index] *= MUL_YEAR;
                }
                */
            }
            index += 1;
        }
        stones.append(&mut new_stones);
        let duration = start.elapsed();
        println!("Finished #{} in {:?}", i + 1, duration);
    }
    stones.len()
}