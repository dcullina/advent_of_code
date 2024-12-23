use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Object {
    EmptySpace,
    Wall,
    Start,
    End,
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

pub fn part_one_two<const CHEAT_LENGTH: usize, const MIN_GAIN: usize>(input: &str) -> usize {
    let mut map: HashMap<(isize, isize), (Object, usize)> = input
        .lines()
        .enumerate()
        .flat_map(|(row_index, line)| {
            line.chars().enumerate().map(move |(col_index, character)| {
                let object: Object = match character {
                    '.' => Object::EmptySpace,
                    '#' => Object::Wall,
                    'S' => Object::Start,
                    'E' => Object::End,
                    _ => unreachable!(),
                };
                (
                    (row_index as isize, col_index as isize),
                    (object, usize::MAX),
                )
            })
        })
        .collect();

    let start: (isize, isize) = map
        .iter()
        .find(|(_, value)| value.0 == Object::Start)
        .map(|(&key, _)| key)
        .unwrap();

    let end: (isize, isize) = map
        .iter()
        .find(|(_, value)| value.0 == Object::End)
        .map(|(&key, _)| key)
        .unwrap();

    // set the end position score equal to zero
    // flood the entire map with min distances from end
    map.get_mut(&end).unwrap().1 = 0;
    get_dists_from_end(&mut map, &end);
    let start_dist: usize = map.get(&start).unwrap().1;

    let mut path: HashSet<(isize, isize)> = HashSet::new();
    walk_path(&map, &start, &mut path);
    let mut good_cheat_count: usize = 0;

    for coordinate in &path {
        let min_col: isize = coordinate.1 - CHEAT_LENGTH as isize;
        let max_col: isize = coordinate.1 + CHEAT_LENGTH as isize;
        let min_row: isize = coordinate.0 - CHEAT_LENGTH as isize;
        let max_row: isize = coordinate.0 + CHEAT_LENGTH as isize;

        for new_row in min_row..=max_row {
            for new_col in min_col..=max_col {
                let end_coord = (new_row, new_col);
                let cheated_distance = compute_distance(*coordinate, end_coord);

                if cheated_distance == 0
                    || cheated_distance > CHEAT_LENGTH
                    || !path.contains(&(new_row, new_col))
                {
                    continue;
                }

                let current_score: isize = map.get(coordinate).unwrap().1 as isize;
                let end_score: isize = map.get(&end_coord).unwrap().1 as isize;

                let savings: isize = current_score - end_score - cheated_distance as isize;
                if savings >= MIN_GAIN as isize {
                    good_cheat_count += 1;
                }
            }
        }
    }

    good_cheat_count
}

fn compute_distance(point_a: (isize, isize), point_b: (isize, isize)) -> usize {
    ((point_a.0 - point_b.0).abs() + (point_a.1 - point_b.1).abs()) as usize
}

fn walk_path(
    map: &HashMap<(isize, isize), (Object, usize)>,
    current_position: &(isize, isize),
    path_set: &mut HashSet<(isize, isize)>,
) {
    if map.get(current_position).unwrap().0 == Object::End {
        path_set.insert(*current_position);
        return;
    }

    for direction in DIRECTIONS {
        let next_coord: (isize, isize) = (
            current_position.0 + direction.0,
            current_position.1 + direction.1,
        );
        if let Some(object) = map.get(&next_coord) {
            if object.0 != Object::Wall {
                let next_score: usize = map.get(&next_coord).unwrap().1;
                let current_score: usize = map.get(current_position).unwrap().1;
                if next_score == current_score - 1 {
                    path_set.insert(*current_position);
                    walk_path(map, &next_coord, path_set);
                }
            }
        }
    }
}

fn get_dists_from_end(
    map: &mut HashMap<(isize, isize), (Object, usize)>,
    current_position: &(isize, isize),
) {
    for direction in DIRECTIONS {
        let next_coord: (isize, isize) = (
            current_position.0 + direction.0,
            current_position.1 + direction.1,
        );
        if let Some(object) = map.get(&next_coord) {
            if object.0 != Object::Wall {
                let next_score: usize = map.get(&next_coord).unwrap().1;
                let next_possible_score: usize = map.get(current_position).unwrap().1 + 1;
                if next_possible_score < next_score {
                    map.get_mut(&next_coord).unwrap().1 = next_possible_score;
                    get_dists_from_end(map, &next_coord);
                }
            }
        }
    }
}
