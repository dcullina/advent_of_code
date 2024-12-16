const ROW_SCALER: usize = 100;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Object {
    BoxLeft,
    BoxRight,
    EmptySpace,
    Robot,
    Wall
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn part_two(input: &str) -> usize {
    let mut movements: Vec<Direction> = Vec::new();
    let mut grid: Vec<Vec<Object>> = Vec::new();
    let mut robot_position: (usize, usize) = (0, 0);

    let mut input_parts = input.split("\n\n");

    input_parts.next().unwrap()
        .lines()
        .enumerate()
        .for_each(|(row_index, line)| {
            grid.push(vec![]);
            line
                .chars()
                .enumerate()
                .for_each(|(col_index, character)| {
                    match character {
                        '#' => {
                            grid[row_index].push(Object::Wall);
                            grid[row_index].push(Object::Wall);
                        },
                        '@' => {
                            robot_position = (
                                row_index,
                                col_index * 2
                            );
                            grid[row_index].push(Object::Robot);
                            grid[row_index].push(Object::EmptySpace);
                        },
                        'O' => {
                            grid[row_index].push(Object::BoxLeft);
                            grid[row_index].push(Object::BoxRight);
                        },
                        '.' => {
                            grid[row_index].push(Object::EmptySpace);
                            grid[row_index].push(Object::EmptySpace);
                        }
                        _ => ()
                    };
                });
        });

    input_parts.next().unwrap()
        .lines()
        .for_each(|line| {
            line
                .chars()
                .for_each(|character| {
                    match character {
                        '<' => movements.push(Direction::Left),
                        '>' => movements.push(Direction::Right),
                        '^' => movements.push(Direction::Up),
                        'v' => movements.push(Direction::Down),
                        _ => ()
                    };
                });
        });

    movements
        .iter()
        .for_each(|movement| {
            // println!("{:?}", movement);
            // print_grid(&grid);
            if !check_safety(robot_position, movement, &mut grid) { return; }

            if attempt_move(robot_position, movement, &mut grid) {
                grid[robot_position.0][robot_position.1] = Object::EmptySpace;
                robot_position = get_next_position(&robot_position, movement);
                grid[robot_position.0][robot_position.1] = Object::Robot;
            }
            // wait_for_keypress();
        });

    // print_grid(&grid);

    grid
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row
                .iter()
                .enumerate()
                .filter_map(|(col_index, object)| {
                    match object {
                        Object::BoxLeft => Some((row_index * ROW_SCALER) + col_index),
                        _ => None,
                    }
                }).sum::<usize>()
        })
        .sum::<usize>()
}

fn get_next_position(obj_position: &(usize, usize), direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => {
            (
                obj_position.0 - 1,
                obj_position.1
            )
        },
        Direction::Right => {
            (
                obj_position.0,
                obj_position.1 + 1
            )
        },
        Direction::Down => {
            (
                obj_position.0 + 1,
                obj_position.1
            )
        },
        Direction::Left => {
            (
                obj_position.0,
                obj_position.1 - 1
            )
        }
    }
}

fn check_safety(obj_position: (usize, usize), direction: &Direction, grid: &mut Vec<Vec<Object>>) -> bool {

    let next_position = get_next_position(&obj_position, direction);
    let next_object: Object = grid[next_position.0][next_position.1];

    match (next_object, direction) {
        (Object::EmptySpace, _) => {
            return true;
        },
        (Object::BoxLeft | Object::BoxRight, Direction::Right | Direction::Left) => {
            check_safety(next_position, direction, grid)
        },
        (Object::BoxLeft, Direction::Up | Direction::Down) => {
            check_safety(next_position, direction, grid)
                && check_safety((next_position.0, next_position.1 + 1), direction, grid)
        },
        (Object::BoxRight, Direction::Up | Direction::Down) => {
            check_safety(next_position, direction, grid)
                && check_safety((next_position.0, next_position.1 - 1), direction, grid)
        },
        (_, _) => false
    }
}

fn attempt_move(obj_position: (usize, usize), direction: &Direction, grid: &mut Vec<Vec<Object>>) -> bool {

    let next_position = get_next_position(&obj_position, direction);
    let next_object: Object = grid[next_position.0][next_position.1];

    match (next_object, direction) {
        (Object::EmptySpace, _) => {
            return true;
        },
        (Object::BoxLeft | Object::BoxRight, Direction::Right | Direction::Left) => {
            if attempt_move(next_position, direction, grid) {
                let nextnext_position: (usize, usize) = get_next_position(&next_position, direction);
                grid[next_position.0][next_position.1] = Object::EmptySpace;

                grid[nextnext_position.0][nextnext_position.1] = next_object;
                return true;
            }
        },
        (Object::BoxLeft, Direction::Up | Direction::Down) => {
            if attempt_move(next_position, direction, grid)
                && attempt_move((next_position.0, next_position.1 + 1), direction, grid)
            {
                let nextnext_position: (usize, usize) = get_next_position(&next_position, direction);
                grid[next_position.0][next_position.1] = Object::EmptySpace;
                grid[next_position.0][next_position.1 + 1] = Object::EmptySpace;

                grid[nextnext_position.0][nextnext_position.1] = Object::BoxLeft;
                grid[nextnext_position.0][nextnext_position.1 + 1] = Object::BoxRight;
                return true;
            }
        },
        (Object::BoxRight, Direction::Up | Direction::Down) => {
            if attempt_move(next_position, direction, grid)
                && attempt_move((next_position.0, next_position.1 - 1), direction, grid)
            {
                let nextnext_position: (usize, usize) = get_next_position(&next_position, direction);
                grid[next_position.0][next_position.1 - 1] = Object::EmptySpace;
                grid[next_position.0][next_position.1] = Object::EmptySpace;

                grid[nextnext_position.0][nextnext_position.1 - 1] = Object::BoxLeft;
                grid[nextnext_position.0][nextnext_position.1] = Object::BoxRight;
                return true;
            }
        },
        (_, _) => ()
    };

    false
}

fn wait_for_keypress() {
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn print_grid(grid: &Vec<Vec<Object>>) {

    for row_index in 0..grid.len() {
        for col_index in 0..grid[0].len() {
            let symbol = match grid[row_index][col_index] {
                Object::Wall => '#',
                Object::Robot => '@',
                Object::EmptySpace => '.',
                Object::BoxLeft => '[',
                Object::BoxRight => ']',
            };
            print!("{}", symbol);
        }
        println!();
    }
}