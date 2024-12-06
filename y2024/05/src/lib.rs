use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

pub fn part_one(input: &str) -> i32 {
    let input_parts: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<Vec<i32>> = input_parts[0]
        .lines()
        .filter_map(|line: &str| {
            line.split('|')
                .map(|number: &str| number.parse::<i32>().ok())
                .collect()
        })
        .collect(); // the number rules, e.g., 47|91

    let orders: Vec<Vec<i32>> = input_parts[1]
        .lines()
        .map(|line: &str| {
            line.split(',')
                .filter_map(|number: &str| number.parse::<i32>().ok())
                .collect()
        })
        .collect();

    // create a hash-set of all numbers rules
    let mut rule_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    rules.iter().for_each(|rule: &Vec<i32>| {
        rule_map
            .entry(rule[0])
            .or_insert_with(HashSet::new)
            .insert(rule[1]);
    });

    orders
        .iter()
        .filter(|order: &&Vec<i32>| {
            order.is_sorted_by(|a: &i32, b: &i32| {
                if let Some(set) = rule_map.get(b) {
                    !set.contains(a)
                } else {
                    true
                }
            })
        })
        .map(|order: &Vec<i32>| {
            order[order.len() / 2]
        }).sum()

}

pub fn part_two(input: &str) -> i32 {
    let input_parts: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<Vec<i32>> = input_parts[0]
        .lines()
        .filter_map(|line: &str| {
            line.split('|')
                .map(|number: &str| number.parse::<i32>().ok())
                .collect()
        })
        .collect(); // the number rules, e.g., 47|91

    let orders: Vec<Vec<i32>> = input_parts[1]
        .lines()
        .map(|line| {
            line.split(',')
                .filter_map(|number: &str| number.parse::<i32>().ok())
                .collect()
        })
        .collect();

    // create a hash-set of all numbers rules
    let mut rule_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    rules.iter().for_each(|rule: &Vec<i32>| {
        rule_map
            .entry(rule[0])
            .or_insert_with(HashSet::new)
            .insert(rule[1]);
    });

    // loop through all orders
    orders
        .iter()
        .filter(|order: &&Vec<i32>| {
            !order.is_sorted_by(|a: &i32, b: &i32| {
                if let Some(set) = rule_map.get(b) {
                    !set.contains(a)
                } else {
                    true
                }
            })
        })
        .map(|order: &Vec<i32>| {
            let mut new_order: Vec<i32> = order.clone();
            new_order.sort_by(|a: &i32, b: &i32| {
                if let Some(set) = rule_map.get(b) {
                    if set.contains(a) {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            });
            new_order[new_order.len() / 2]
        }).sum()
}
