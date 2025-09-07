use rayon::prelude::*;
use std::collections::HashSet;

pub fn part_one(input: &str) -> i32 {
    // get grid from input
    let grid: Vec<Vec<char>> = load_map_from_str(input);

    // remove repeats where there may be multiple directions for a single position
    get_path_from_grid(&grid)
        .0
        .into_iter()
        .map(|(row, column, _)| (row, column))
        .collect::<HashSet<(usize, usize)>>()
        .len() as i32
}

pub fn part_two(input: &str) -> i32 {
    // get grid from input
    let grid: Vec<Vec<char>> = load_map_from_str(input);

    // get set of locations that were visited by walking the grid
    let unique_positions: HashSet<(usize, usize)> = get_path_from_grid(&grid)
        .0
        .into_iter()
        .map(|(row, column, _)| (row, column))
        .collect::<HashSet<(usize, usize)>>();

    unique_positions
        .par_iter()
        .map(|&(row, column)| {
            let mut new_grid: Vec<Vec<char>> = grid.clone();
            new_grid[row][column] = 'O';

            let loop_detected: bool = get_path_from_grid(&new_grid).1;
            if loop_detected {
                1
            } else {
                0
            }
        })
        .sum()
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn load_map_from_str(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_path_from_grid(grid: &Vec<Vec<char>>) -> (HashSet<(usize, usize, Direction)>, bool) {
    let grid_size: (usize, usize) = (grid.len(), grid[0].len());

    let mut guard_position: (usize, usize) = grid
        .iter()
        .enumerate()
        .find_map(|(row_index, row)| {
            row.iter()
                .position(|&character| character == '^')
                .map(|column_index: usize| (row_index, column_index))
        })
        .unwrap_or((0, 0));

    let mut visited: HashSet<(usize, usize, Direction)> = HashSet::new();

    let mut direction: Direction = Direction::Up;

    let mut loop_detected: bool = false;

    loop {
        // record the guard's position
        visited.insert((guard_position.0, guard_position.1, direction));

        // get the guard's next position
        let next_position: Option<(usize, usize)> =
            get_next_position(&direction, &guard_position, &grid_size);

        match next_position {
            Some(position) => {
                if visited.contains(&(position.0, position.1, direction)) {
                    loop_detected = true;
                    break;
                }
                match grid[position.0][position.1] {
                    '.' | '^' => {
                        guard_position = position;
                    }
                    '#' | 'O' => {
                        direction = match direction {
                            Direction::Up => Direction::Right,
                            Direction::Right => Direction::Down,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Up,
                        }
                    }
                    _ => {}
                }
            }
            None => {
                break;
            }
        }
    }

    (visited, loop_detected)
}

fn get_next_position(
    direction: &Direction,
    guard_position: &(usize, usize),
    grid_size: &(usize, usize),
) -> Option<(usize, usize)> {
    let guard_position: (isize, isize) = (guard_position.0 as isize, guard_position.1 as isize);
    let grid_size: (isize, isize) = (grid_size.0 as isize, grid_size.1 as isize);

    let next_position: (isize, isize) = match direction {
        Direction::Up => (guard_position.0 - 1, guard_position.1),
        Direction::Right => (guard_position.0, guard_position.1 + 1),
        Direction::Down => (guard_position.0 + 1, guard_position.1),
        Direction::Left => (guard_position.0, guard_position.1 - 1),
    };

    // check if out of bounds
    if (next_position.0 < 0)
        || (next_position.1 < 0)
        || (next_position.0 >= grid_size.0)
        || (next_position.1 >= grid_size.1)
    {
        return None;
    }

    Some((next_position.0 as usize, next_position.1 as usize))
}
