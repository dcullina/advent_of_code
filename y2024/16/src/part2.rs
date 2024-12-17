use crate::part1::{
    dfs_explore,
    get_next_coord,
    Direction,
    Object
};
use std::usize::MAX;
use std::collections::HashSet;

pub fn part_two(input: &str) -> usize {
    let mut grid: Vec<Vec<Object>> = Vec::new();
    let mut start_pos: (usize, usize) = (0, 0);
    let mut end_pos: (usize, usize) = (0, 0);

    input
        .lines()
        .enumerate()
        .for_each(|(row_index, line)| {
            grid.push(vec![]);
            line
                .chars()
                .enumerate()
                .for_each(|(col_index, character)| {
                    match character {
                        '#' => grid[row_index].push(Object::Wall),
                        'S' => {
                            start_pos = (row_index, col_index);
                            grid[row_index].push(Object::Start)
                        },
                        'E' => {
                            end_pos = (row_index, col_index);
                            grid[row_index].push(Object::EmptySpace)
                        },
                        '.' => grid[row_index].push(Object::EmptySpace),
                        _ => ()
                    };
                });
        });

    let mut score_grid: Vec<Vec<usize>> = vec![vec![MAX; grid[0].len()]; grid.len()];
    score_grid[start_pos.0][start_pos.1] = 0;
    dfs_explore(&grid, &mut score_grid, &start_pos, &Direction::East);

    let mut best_positions: HashSet<(usize, usize)> = HashSet::new();
    walk_back(&mut best_positions, &grid, &score_grid, &end_pos, &Direction::South, &start_pos); // 50/50 guess on where the reindeer is coming from lmao
    best_positions.len()
}

fn walk_back(
    best_positions: &mut HashSet<(usize, usize)>,
    grid: &Vec<Vec<Object>>,
    score_grid: &Vec<Vec<usize>>,
    current_position: &(usize, usize),
    current_dir: &Direction,
    start_pos: &(usize, usize)
) {
    if best_positions.contains(current_position) { return; }
    // print_grid(grid, best_positions);

    best_positions.insert(*current_position);

    if current_position == start_pos { return; }

    let directions: [((usize, usize), Direction); 4] = [
        (get_next_coord(current_position, &Direction::South), Direction::South),
        (get_next_coord(current_position, &Direction::West), Direction::West),
        (get_next_coord(current_position, &Direction::East), Direction::East),
        (get_next_coord(current_position, &Direction::North), Direction::North),
    ];

    let scores: Vec<usize> = directions
        .iter()
        .map(|(coordinate, direction)| {
            if grid[coordinate.0][coordinate.1] == Object::Wall { return MAX; }
            if current_dir == direction {
                score_grid[coordinate.0][coordinate.1]
            } else if get_opposite(direction) == *current_dir {
                MAX
            } else {
                score_grid[coordinate.0][coordinate.1] + 1000
            }
        }).collect();

    

    // println!("South: {}\tWest: {}\tEast: {}\tNorth: {}", scores[0], scores[1], scores[2], scores[3]);

    let min_score: usize = *scores.iter().min().unwrap();
    if min_score == MAX { return; }

    scores
        .iter()
        .enumerate()
        .for_each(|(dir, score)| {
            if *score == min_score {
                walk_back(best_positions, grid, score_grid, &directions[dir].0, &directions[dir].1, start_pos);
            }
        })

}

fn print_grid(grid: &Vec<Vec<Object>>, best_positions: &HashSet<(usize, usize)>) {
    grid
        .iter()
        .enumerate()
        .for_each(|(row_index, row)| {
            row
                .iter()
                .enumerate()
                .for_each(|(col_index, element)| {
                    let symbol: char = match element {
                        Object::EmptySpace => {
                            if best_positions.contains(&(row_index, col_index)) {
                                'O'
                            } else {
                                '.'
                            }
                        },
                        Object::Start => {
                            if best_positions.contains(&(row_index, col_index)) {
                                'O'
                            } else {
                                'S'
                            }
                        },
                        Object::Wall => {
                            if best_positions.contains(&(row_index, col_index)) {
                                'O'
                            } else {
                                '#'
                            }
                        },
                    };
                    print!("{}", symbol);
                });
            println!();
        });
}

fn get_opposite(direction: &Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
        Direction::East => Direction::West,
    }
}