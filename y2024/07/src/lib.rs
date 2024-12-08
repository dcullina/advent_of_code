use rayon::prelude::*;

pub fn part_one(input: &str) -> u64 {
    let input_set: Vec<(u64, Vec<u64>)> = input.replace(":","")
        .lines()
        .map(|line: &str| {
            let row: Vec<u64> = line.split_whitespace()
                .filter_map(|number: &str| number.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            (row[0], row[1..].to_vec())
        })
        .collect::<Vec<(u64, Vec<u64>)>>();


    input_set.iter()
        .map(|line: &(u64, Vec<u64>)| {
            // if check_valid_one(&line.0, &line.1) {
            if recursive_one(line.1[1..].to_vec(), line.1[0], line.0) {
                line.0
            } else {
                0
            }
        }).sum::<u64>()
}

fn recursive_one(remaining_numbers: Vec<u64>, current_result: u64, target: u64) -> bool {
    if remaining_numbers.is_empty() {
        return current_result == target;
    }

    let next_value: u64 = remaining_numbers[0];
    let new_remaining = &remaining_numbers[1..];

    if recursive_one(new_remaining.to_vec(), current_result + next_value, target) {
        return true;
    }

    if recursive_one(new_remaining.to_vec(), current_result * next_value, target) {
        return true;
    }

    false
}

pub fn part_two(input: &str) -> u64 {
    let input_set: Vec<(u64, Vec<u64>)> = input.replace(":","")
        .lines()
        .map(|line: &str| {
            let row: Vec<u64> = line.split_whitespace()
                .filter_map(|number: &str| number.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            (row[0], row[1..].to_vec())
        })
        .collect::<Vec<(u64, Vec<u64>)>>();


    input_set.par_iter()
        .map(|line: &(u64, Vec<u64>)| {
            if recursive_two(line.1[1..].to_vec(), line.1[0], line.0) {
                line.0
            } else {
                0
            }
        }).sum::<u64>()
        
}

fn recursive_two(remaining_numbers: Vec<u64>, current_result: u64, target: u64) -> bool {
    if remaining_numbers.is_empty() {
        return current_result == target;
    }

    let next_value: u64 = remaining_numbers[0];
    let new_remaining = &remaining_numbers[1..];

    if recursive_two(new_remaining.to_vec(), concat_numbers(current_result, next_value), target) {
        return true;
    }

    if recursive_two(new_remaining.to_vec(), current_result + next_value, target) {
        return true;
    }

    if recursive_two(new_remaining.to_vec(), current_result * next_value, target) {
        return true;
    }


    false
}

pub fn concat_numbers(left: u64, right: u64) -> u64 {
    left * 10u64.pow(
        (right as f64).log10().floor() as u32 + 1
    ) + right
}