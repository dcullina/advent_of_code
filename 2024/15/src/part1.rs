use std::collections::HashMap;

const ROW_SCALER: usize = 100;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Coordinate {
    row: isize,
    column: isize
}

impl Coordinate {
    fn new(row: isize, column: isize) -> Self {
        Self {
            row,
            column
        }
    }

    fn compute_gps_score(&self) -> usize {
        (ROW_SCALER * self.row as usize)  + self.column as usize
    }

    fn move_in_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => Self {
                row: self.row - 1,
                column: self.column
            },
            Direction::Right => Self {
                row: self.row,
                column: self.column + 1
            },
            Direction::Down => Self {
                row: self.row + 1,
                column: self.column
            },
            Direction::Left => Self {
                row: self.row,
                column: self.column - 1
            }
        }
    }

}

#[derive(Debug, PartialEq)]
enum Object {
    Box,
    EmptySpace,
    Robot,
    Wall
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub fn part_one(input: &str) -> usize {
    let mut movements: Vec<Direction> = Vec::new();
    let mut coordinates: HashMap<Coordinate, Object> = HashMap::new();
    let mut robot_position: Coordinate = Coordinate::new(0, 0);

    let mut input_parts = input.split("\n\n");
    input_parts.next().unwrap()
        .lines()
        .enumerate()
        .for_each(|(row_index, line)| {
            line
                .chars()
                .enumerate()
                .for_each(|(col_index, character)| {
                    let coord: Coordinate = Coordinate::new(row_index as isize, col_index as isize);
                    match character {
                        '#' => coordinates.insert(coord, Object::Wall),
                        '@' => {
                            robot_position = coord.clone();
                            coordinates.insert(coord, Object::Robot)
                        },
                        'O' => coordinates.insert(coord, Object::Box),
                        '.' => coordinates.insert(coord, Object::EmptySpace),
                        _ => None
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
            if attempt_move(robot_position, movement, &mut coordinates) {
                *coordinates.get_mut(&robot_position).unwrap() = Object::EmptySpace;
                robot_position = robot_position.move_in_direction(movement);
                *coordinates.get_mut(&robot_position).unwrap() = Object::Robot;
            }
        });
    

    coordinates
        .iter()
        .filter_map(|(coordinate, object)| match object {
            Object::Box => Some(coordinate.compute_gps_score()),
            _ => None,
        })
        .sum::<usize>()
}

fn attempt_move(obj_position: Coordinate, direction: &Direction, coordinate_map: &mut HashMap<Coordinate, Object>) -> bool {

    let next_position = obj_position.move_in_direction(&direction);
    match coordinate_map.get(&next_position) {
        Some(&Object::EmptySpace) => {
            return true;
        },
        Some(&Object::Box) => {
            if attempt_move(next_position, direction, coordinate_map) {
                *coordinate_map.get_mut(&next_position.move_in_direction(direction)).unwrap() = Object::Box;
                return true;
            }
        },
        _ => ()
    };

    false
}

fn print_grid(coordinate_map: &HashMap<Coordinate, Object>) {
    let max_row = coordinate_map.keys().map(|coord| coord.row).max().unwrap_or(0) as usize + 1;
    let max_col = coordinate_map.keys().map(|coord| coord.column).max().unwrap_or(0) as usize + 1;

    for row in 0..max_row {
        for col in 0..max_col {
            let coord = Coordinate::new(row as isize, col as isize);
            let symbol = match coordinate_map.get(&coord) {
                Some(Object::Wall) => '#',
                Some(Object::Robot) => '@',
                Some(Object::EmptySpace) => '.',
                Some(Object::Box) => 'O',
                None => ' ',
            };
            print!("{}", symbol);
        }
        println!();
    }
}