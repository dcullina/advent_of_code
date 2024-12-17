use std::usize::MAX;

#[derive(PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West
}

#[derive(PartialEq)]
pub enum Object {
    EmptySpace,
    Wall,
    Start,
}

pub fn part_one(input: &str) -> usize {
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
    score_grid[end_pos.0][end_pos.1]
}

pub fn dfs_explore(grid: &Vec<Vec<Object>>,score_grid: &mut Vec<Vec<usize>>, current_position: &(usize, usize), direction: &Direction) {
    // move north
    let up: (usize, usize) = get_next_coord(current_position, &Direction::North);
    match grid[up.0][up.1] {
        Object::EmptySpace => {
            match direction {
                Direction::South => (),
                Direction::East | Direction::West => {
                    let potential_next_score: usize = score_grid[current_position.0][current_position.1] + 1001;
                    let next_score: usize = score_grid[up.0][up.1];
                    if next_score > potential_next_score {
                        score_grid[up.0][up.1] = potential_next_score;
                        dfs_explore(grid, score_grid, &up, &Direction::North);
                    }
                },
                Direction::North => {
                    let potential_next_score: usize = score_grid[current_position.0][current_position.1] + 1;
                    let next_score: usize = score_grid[up.0][up.1];
                    if next_score > potential_next_score {
                        score_grid[up.0][up.1] = potential_next_score;
                        dfs_explore(grid, score_grid, &up, direction);
                    }
                }
            }
        },
        _ => ()
    }

    // move down
    let down: (usize, usize) = get_next_coord(current_position, &Direction::South);
    match grid[down.0][down.1] {
        Object::EmptySpace => {
            match direction {
                Direction::North => (),
                Direction::East | Direction::West => {
                    let potential_next_score: usize = score_grid[current_position.0][current_position.1] + 1001;
                    let next_score: usize = score_grid[down.0][down.1];
                    if next_score > potential_next_score {
                        score_grid[down.0][down.1] = potential_next_score;
                        dfs_explore(grid, score_grid, &down, &Direction::South);
                    }
                },
                Direction::South => {
                    let potential_next_score: usize = score_grid[current_position.0][current_position.1] + 1;
                    let next_score: usize = score_grid[down.0][down.1];
                    if next_score > potential_next_score {
                        score_grid[down.0][down.1] = potential_next_score;
                        dfs_explore(grid, score_grid, &down, direction);
                    }
                }
            }
        },
        _ => ()
    }

    // move east
    let right: (usize, usize) = get_next_coord(current_position, &Direction::East);
    match grid[right.0][right.1] {
        Object::EmptySpace => {
            match direction {
                Direction::West => (),
                Direction::North | Direction::South => {
                    let potential_next_score: usize = score_grid[current_position.0][current_position.1] + 1001;
                    let next_score: usize = score_grid[right.0][right.1];
                    if next_score > potential_next_score {
                        score_grid[right.0][right.1] = potential_next_score;
                        dfs_explore(grid, score_grid, &right, &Direction::East);
                    }
                },
                Direction::East => {
                    let potential_next_score: usize = score_grid[current_position.0][current_position.1] + 1;
                    let next_score: usize = score_grid[right.0][right.1];
                    if next_score > potential_next_score {
                        score_grid[right.0][right.1] = potential_next_score;
                        dfs_explore(grid, score_grid, &right, direction);
                    }
                }
            }
        },
        _ => ()
    }

    // move west
    let left: (usize, usize) = get_next_coord(current_position, &Direction::West);
    match grid[left.0][left.1] {
        Object::EmptySpace => {
            match direction {
                Direction::East => (),
                Direction::North | Direction::South => {
                    let potential_next_score: usize = score_grid[current_position.0][current_position.1] + 1001;
                    let next_score: usize = score_grid[left.0][left.1];
                    if next_score > potential_next_score {
                        score_grid[left.0][left.1] = potential_next_score;
                        dfs_explore(grid, score_grid, &left, &Direction::West);
                    }
                },
                Direction::West => {
                    let potential_next_score: usize = score_grid[current_position.0][current_position.1] + 1;
                    let next_score: usize = score_grid[left.0][left.1];
                    if next_score > potential_next_score {
                        score_grid[left.0][left.1] = potential_next_score;
                        dfs_explore(grid, score_grid, &left, direction);
                    }
                }
            }
        },
        _ => ()
    }
}

pub fn get_next_coord(coordinate: &(usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::North => (
            coordinate.0 - 1,
            coordinate.1
        ),
        Direction::East => (
            coordinate.0,
            coordinate.1 + 1
        ),
        Direction::South => (
            coordinate.0 + 1,
            coordinate.1
        ),
        Direction::West => (
            coordinate.0,
            coordinate.1 - 1
        )
    }
}