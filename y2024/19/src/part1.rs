use chrono::Local;
use rayon::prelude::*;
use std::collections::HashSet;

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

    // let mut sum: usize = 0;
    // let mut count: usize = 0;
    // for towel_order in towel_orders {
    //     println!("Inspecting towel order #: {:?}", count);
    //     count += 1;
    //     if check_if_possible(&towels, towel_order) {
    //         sum += 1;
    //     }
    // }
    // sum

    towel_orders
        .par_iter()
        .filter(|towel| {
            // println!("Inspecting towel order, {:?}", Local::now());
            check_if_possible(&towels, towel)
        })
        .count()
}

fn check_if_possible(towels: &HashSet<&str>, towel_order: &str) -> bool {
    // println!("{:?}", towel_order);
    let mut end_i: usize = 0;
    let towel_order_len: usize = towel_order.len();

    if towel_order_len <= 1 {
        return towels.contains(towel_order);
    }

    while end_i < towel_order_len {
        let matching: &str = &towel_order[..=end_i];

        if towels.contains(matching) {
            if end_i == towel_order_len - 1 {
                return true;
            } else if check_if_possible(towels, &towel_order[end_i + 1..]) {
                return true;
            }
        }

        end_i += 1;
    }

    false
}
