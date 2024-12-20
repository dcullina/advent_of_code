use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> usize {
    let mut split_input = input.split("\n\n");

    let mut towels: HashSet<&str> = HashSet::new();
    split_input.next().unwrap().split(", ").for_each(|towel| {
        towels.insert(towel);
    });

    let mut towel_orders: Vec<&str> = Vec::new();
    split_input.next().unwrap().lines().for_each(|line| {
        towel_orders.push(line);
    });

    towel_orders
        .par_iter()
        .filter(|&towel_order| {
            let mut previously_seen: HashMap<&str, bool> = HashMap::new();
            check_if_possible(&towels, towel_order, &mut previously_seen)
        })
        .count()
}

fn check_if_possible<'a>(
    towels: &HashSet<&'a str>,
    towel_order: &'a str,
    previously_seen: &mut HashMap<&'a str, bool>,
) -> bool {
    let towel_order_len: usize = towel_order.len();

    for end in 1..=towel_order_len {
        let front: &str = &towel_order[..end];

        if towels.contains(front) {
            let back = &towel_order[end..];
            if back.is_empty() || check_if_possible(towels, back, previously_seen) {
                previously_seen.insert(towel_order, true);
                return true;
            }
        }
    }

    previously_seen.insert(towel_order, false);
    false
}
