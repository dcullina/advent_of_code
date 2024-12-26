use crate::part1::compute_next_secret;
use std::collections::{HashMap, HashSet};

pub fn part_two<const NUM_NEW_SECRET_NUMBERS: usize>(input: &str) -> usize {
    let buyers: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            let mut secret_value = line.parse::<usize>().unwrap();
            let mut buyer_secrets: Vec<usize> = Vec::new();
            buyer_secrets.push(secret_value);
            for _ in 0..NUM_NEW_SECRET_NUMBERS {
                secret_value = compute_next_secret(secret_value);
                buyer_secrets.push(secret_value);
            }
            buyer_secrets
        })
        .collect();

    solve(buyers)
}

fn solve(buyers: Vec<Vec<usize>>) -> usize {
    let mut order_map: HashMap<Vec<isize>, usize> = HashMap::new();

    for buyer in buyers {
        let diff_vec: Vec<isize> = buyer
            .windows(2)
            .map(|window| (window[1] % 10) as isize - (window[0] % 10) as isize)
            .collect();

        let mut index: usize = 4;
        let mut sequence_prev_seen: HashSet<Vec<isize>> = HashSet::new();

        for window in diff_vec.windows(4) {
            let key: Vec<isize> = window.into();

            if sequence_prev_seen.contains(&key) {
                index += 1;
                continue;
            } else {
                sequence_prev_seen.insert(key.clone());
            }

            let banana_value: usize = buyer[index] % 10;
            *order_map.entry(key).or_insert(0) += banana_value;
            index += 1;
        }
    }

    *order_map.values().max().unwrap_or(&0)
}
