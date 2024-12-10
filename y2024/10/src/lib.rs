use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

pub fn part_one(input: &str) -> usize {
    let mut trail_set: HashSet<(usize, usize)> = HashSet::new();

    let map: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            line
                .chars()
                .enumerate()
                .map(|(col_index, number)| {
                    let value: usize = number.to_digit(10).unwrap() as usize;
                    if value == 0 {
                        trail_set.insert((row_index, col_index));
                    }
                    value
                })
                .collect()
        })
        .collect();

    trail_set
        .par_iter()
        .map(|(row_index, col_index)| {
            let mut seen_set: HashSet<(usize, usize)> = HashSet::new();
            dfs_p1(&map, *row_index, *col_index, 0, &mut seen_set)
        }).sum()
}

fn dfs_p1(map: &Vec<Vec<usize>>, row_index: usize, col_index: usize, level: usize, seen_set: &mut HashSet<(usize, usize)>) -> usize {
    if level == 9 { 

        if seen_set.contains(&(row_index, col_index)) {
            return 0;
        }
        seen_set.insert((row_index, col_index));
        return 1;
    }

    let mut sum: usize = 0;

    let next_level: usize = level + 1;
    /*
     - - > Right
    | 0 0 0 row 0
    | 0 0 0
    V 0 0 0 row m
    Down
    */

    // Check Down - increasing row
    if let Some(row) = map.get(row_index + 1) { // safely get the next index
        if row[col_index] == next_level {
            sum += dfs_p1(map, row_index + 1, col_index, next_level, seen_set);
        }
    }
    // Check Up - decreasing row
    if let Some(new_row_index) = row_index.checked_sub(1) {
        if map[new_row_index][col_index] == next_level {
            sum += dfs_p1(map, new_row_index, col_index, next_level, seen_set);
        }
    }
    // Check Right - increasing column
    if let Some(element) = map[row_index].get(col_index + 1) {
        if *element == next_level {
            sum += dfs_p1(map, row_index, col_index + 1, next_level, seen_set);
        }
    }
    // Check Left - decreasing column
    if let Some(new_col_index) = col_index.checked_sub(1) {
        if map[row_index][new_col_index] == next_level {
            sum += dfs_p1(map, row_index, new_col_index, next_level, seen_set);
        }
    }


    sum
}



pub fn part_two(input: &str) -> usize {
    let mut trail_set: HashSet<(usize, usize)> = HashSet::new();

    let map: Vec<Vec<usize>> = input
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            line
                .chars()
                .enumerate()
                .map(|(col_index, number)| {
                    let value: usize = number.to_digit(10).unwrap() as usize;
                    if value == 0 {
                        trail_set.insert((row_index, col_index));
                    }
                    value
                })
                .collect()
        })
        .collect();

    trail_set
        .par_iter()
        .map(|(row_index, col_index)| {
            dfs_p2(&map, *row_index, *col_index, 0)
        }).sum()
}

fn dfs_p2(map: &Vec<Vec<usize>>, row_index: usize, col_index: usize, level: usize) -> usize {
    if level == 9 { 
        return 1;
    }

    let mut sum: usize = 0;

    let next_level: usize = level + 1;
    /*
     - - > Right
    | 0 0 0 row 0
    | 0 0 0
    V 0 0 0 row m
    Down
    */

    // Check Down - increasing row
    if let Some(row) = map.get(row_index + 1) { // safely get the next index
        if row[col_index] == next_level {
            sum += dfs_p2(map, row_index + 1, col_index, next_level);
        }
    }
    // Check Up - decreasing row
    if let Some(new_row_index) = row_index.checked_sub(1) {
        if map[new_row_index][col_index] == next_level {
            sum += dfs_p2(map, new_row_index, col_index, next_level);
        }
    }
    // Check Right - increasing column
    if let Some(element) = map[row_index].get(col_index + 1) {
        if *element == next_level {
            sum += dfs_p2(map, row_index, col_index + 1, next_level);
        }
    }
    // Check Left - decreasing column
    if let Some(new_col_index) = col_index.checked_sub(1) {
        if map[row_index][new_col_index] == next_level {
            sum += dfs_p2(map, row_index, new_col_index, next_level);
        }
    }


    sum
}