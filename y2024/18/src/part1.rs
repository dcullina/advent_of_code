use std::collections::{HashMap, HashSet};
use std::usize::MAX;

enum Direction {
    Up,
    Right,
    Left,
    Down
}


pub fn part_one(input: &str, grid_dim: usize, num_bytes: usize) -> usize {
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

    for i in 0..num_bytes {
        grid.insert(bytes[i]);
    }

    let mut min_scores: HashMap<(usize, usize), usize> = HashMap::new();
    min_scores.insert((0, 0), 0);
    min_steps(&grid, &mut min_scores, &(grid_dim + 1), &(0, 0), 0); // add 1 since the grid is (GRID_DIM + 1) x (GRID_DIM + 1) spaces
    *min_scores.get(&(grid_dim, grid_dim)).unwrap()
}

fn min_steps(
    grid: &HashSet<(usize, usize)>,
    min_scores: &mut HashMap<(usize, usize), usize>,
    grid_dim: &usize,
    current_pos: &(usize, usize),
    current_score: usize
) {

    if let Some(up) = get_next_coord_safely(grid, current_pos, &Direction::Up, &grid_dim) {
        if grid.contains(&up) {
            // do nothing
        }
        else if min_scores.contains_key(&up) {
            if *min_scores.get(&up).unwrap() > current_score + 1 {
                min_scores.insert(up, current_score + 1);
                min_steps(grid, min_scores, grid_dim, &up, current_score + 1);
            }
        } else {
            min_scores.insert(up, current_score + 1);
            min_steps(grid, min_scores, grid_dim, &up, current_score + 1);
        }
    }
    if let Some(down) = get_next_coord_safely(grid, current_pos, &Direction::Down, &grid_dim) {
        if grid.contains(&down) {}
        else if min_scores.contains_key(&down) {
            if *min_scores.get(&down).unwrap() > current_score + 1 {
                min_scores.insert(down, current_score + 1);
                min_steps(grid, min_scores, grid_dim, &down, current_score + 1);
            }
        } else {
            min_scores.insert(down, current_score + 1);
            min_steps(grid, min_scores, grid_dim, &down, current_score + 1);
        }
    }
    if let Some(left) = get_next_coord_safely(grid, current_pos, &Direction::Left, &grid_dim) {
        if grid.contains(&left) {}
        else if min_scores.contains_key(&left) {
            if *min_scores.get(&left).unwrap() > current_score + 1 {
                min_scores.insert(left, current_score + 1);
                min_steps(grid, min_scores, grid_dim, &left, current_score + 1);
            }
        } else {
            min_scores.insert(left, current_score + 1);
            min_steps(grid, min_scores, grid_dim, &left, current_score + 1);
        }
    }
    if let Some(right) = get_next_coord_safely(grid, current_pos, &Direction::Right, &grid_dim) {
        if grid.contains(&right) {}
        else if min_scores.contains_key(&right) {
            if *min_scores.get(&right).unwrap() > current_score + 1 {
                min_scores.insert(right, current_score + 1);
                min_steps(grid, min_scores, grid_dim, &right, current_score + 1);
            }
        } else {
            min_scores.insert(right, current_score + 1);
            min_steps(grid, min_scores, grid_dim, &right, current_score + 1);
        }
    }
}

fn get_next_coord_safely(
    grid: &HashSet<(usize, usize)>,
    current_pos: &(usize, usize),
    direction: &Direction,
    grid_dim: &usize
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            current_pos.0.checked_sub(1).map(|new_row| (new_row, current_pos.1))
        },
        Direction::Right => {
            if current_pos.1 + 1 < *grid_dim {
                Some((current_pos.0, current_pos.1 + 1))
            } else {
                None
            }
        },
        Direction::Left => {
            current_pos.1.checked_sub(1).map(|new_column| (current_pos.0, new_column))
        },
        Direction::Down => {
            if current_pos.0 + 1 < *grid_dim {
                Some((current_pos.0 + 1, current_pos.1))
            } else {
                None
            }
        }
    }
}