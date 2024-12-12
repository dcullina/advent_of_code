use std::time::Instant;
use std::collections::VecDeque;

const MUL_YEAR: usize = 2024;

pub fn part_one_two(input: &str, n_blinks: usize) -> usize {
    let start = Instant::now();
    let mut initial_stones: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|number| {
            number.parse::<usize>().unwrap()
        })
        .collect();

    let mut total_stones: usize = 0;
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for stone in initial_stones.drain(..) {
        queue.push_back((stone, n_blinks));
    }

    while let Some((mut value, remaining_blinks)) = queue.pop_front() {
        if remaining_blinks == 0 {
            total_stones += 1;
            continue;
        }

        if value == 0 {
            queue.push_back((1, remaining_blinks - 1));
        } else {
            let digits = value.to_string();
            let digits_count = digits.len();

            if digits_count % 2 == 0 {
                let mid = digits_count / 2;
                let left = digits[..mid].parse::<usize>().unwrap();
                let right = digits[mid..].parse::<usize>().unwrap();

                queue.push_back((left, remaining_blinks - 1));
                queue.push_back((right, remaining_blinks - 1));
            } else {
                value *= MUL_YEAR;
                queue.push_back((value, remaining_blinks - 1));
            }
        }
    }

    let duration = start.elapsed();
    println!("Finished Execution in: {:?}", duration);
    total_stones
}