use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn part_two(input: &str) -> usize {
    let mut split_input = input.split("\n\n");

    let mut towels: HashSet<&str> = HashSet::new();
    split_input.next().unwrap().split(", ").for_each(|towel| {
        towels.insert(towel);
    });

    let towel_orders: Vec<&str> = split_input.next().unwrap().lines().collect();

    towel_orders
        .par_iter()
        .map(|&towel_order| {
            let mut previously_seen: HashMap<&str, usize> = HashMap::new();
            count_combinations(&towels, towel_order, &mut previously_seen)
        })
        .sum()
}

fn count_combinations<'a>(
    towels: &HashSet<&'a str>,
    towel_order: &'a str,
    previously_seen: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&result) = previously_seen.get(towel_order) {
        return result;
    }

    if towel_order.is_empty() {
        return 1;
    }

    let mut combinations = 0;
    let towel_order_len: usize = towel_order.len();

    for end in 1..=towel_order_len {
        let front: &str = &towel_order[..end];

        if towels.contains(front) {
            let back = &towel_order[end..];
            combinations += count_combinations(towels, back, previously_seen);
        }
    }

    previously_seen.insert(towel_order, combinations);
    combinations
}
