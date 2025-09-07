use std::collections::HashMap;

const MUL_YEAR: usize = 2024;

fn update_stone_counts(stone_count_map: &HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut new_stone_count_map: HashMap<usize, usize> = HashMap::new();
    
    for (&stone_value, count) in stone_count_map {
        if stone_value == 0 {
            *new_stone_count_map.entry(1).or_insert(0) += count;
        } else {
            
            let digits = stone_value.to_string();
            let digits_count = digits.len();

            if digits_count % 2 == 0 {

                let mid = digits_count / 2;
                let left = digits[..mid].parse::<usize>().unwrap();
                let right = digits[mid..].parse::<usize>().unwrap();

                *new_stone_count_map.entry(left).or_insert(0) += count;
                *new_stone_count_map.entry(right).or_insert(0) += count;
              

            } else {
                *new_stone_count_map.entry(stone_value * MUL_YEAR).or_insert(0) += count;
            }
        }
    }

    new_stone_count_map
}

pub fn part_one_two(input: &str, n_blinks: usize) -> usize {

    let mut stone_counts: HashMap<usize, usize> = HashMap::new();

    input
        .split_ascii_whitespace()
        .for_each(|number| {
            let stone_value: usize = number.parse::<usize>().unwrap();
            *stone_counts.entry(stone_value).or_insert(0) += 1;
        });

    for _ in 0..n_blinks {
        
        stone_counts = update_stone_counts(&stone_counts);

    }

    stone_counts.values().sum()
}