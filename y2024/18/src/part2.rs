use std::collections::{HashSet, HashMap};
use crate::part1::*;

pub fn part_two(input: &str, grid_dim: usize, num_bytes: usize) -> String {
    let mut grid: HashSet<(usize, usize)> = HashSet::new();
    let bytes: Vec<(usize, usize)> = input
        .lines()
        .map(|line| {
            let digits: Vec<usize> = line
                .split(',')
                .map(|digit| digit.parse::<usize>().unwrap())
                .collect();
            (digits[0], digits[1])
        })
        .collect::<Vec<(usize, usize)>>();

    for i in 0..bytes.len() {
        println!("Byte num: {:?}", i);
        // setup map
        grid.insert(bytes[i]);
        let mut min_scores: HashMap<(usize, usize), usize> = HashMap::new();
        min_scores.insert((0, 0), 0);
        // run simulation
        min_steps(&grid, &mut min_scores, &(grid_dim + 1), &(0, 0), 0); // add 1 since the grid is (GRID_DIM + 1) x (GRID_DIM + 1) spaces
        if min_scores.get(&(grid_dim, grid_dim)) == None {
            return format!("{},{}", bytes[i].0, bytes[i].1)
        }
    }
    "always an exit".to_string()
}

fn min_steps(
    grid: &HashSet<(usize, usize)>,
    min_scores: &mut HashMap<(usize, usize), usize>,
    grid_dim: &usize,
    current_pos: &(usize, usize),
    current_score: usize
) {
    if *current_pos == (*grid_dim - 1, *grid_dim - 1) { return; }

    if let Some(up) = get_next_coord_safely(grid, current_pos, &Direction::Up, &grid_dim) {
        if !grid.contains(&up) && !min_scores.contains_key(&up) {
            min_scores.insert(up, current_score + 1);
            min_steps(grid, min_scores, grid_dim, &up, current_score + 1);
        }
    }
    if let Some(down) = get_next_coord_safely(grid, current_pos, &Direction::Down, &grid_dim) {
        if !grid.contains(&down) && !min_scores.contains_key(&down) {
            min_scores.insert(down, current_score + 1);
            min_steps(grid, min_scores, grid_dim, &down, current_score + 1);
        }
    }
    if let Some(left) = get_next_coord_safely(grid, current_pos, &Direction::Left, &grid_dim) {
        if !grid.contains(&left) && !min_scores.contains_key(&left) {
            min_scores.insert(left, current_score + 1);
            min_steps(grid, min_scores, grid_dim, &left, current_score + 1);
        }
    }
    if let Some(right) = get_next_coord_safely(grid, current_pos, &Direction::Right, &grid_dim) {
        if !grid.contains(&right) && !min_scores.contains_key(&right) {
            min_scores.insert(right, current_score + 1);
            min_steps(grid, min_scores, grid_dim, &right, current_score + 1);
        }
    }
}