use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Object {
    EmptySpace,
    Wall,
    Start,
    End,
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

pub fn part_one<const CHEAT_LENGTH: usize, const MIN_GAIN: usize>(input: &str) -> usize {
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

    // set the start position score equal to zero
    map.get_mut(&start).unwrap().1 = 0;

    // get the fastest possible time
    let best: isize = {
        let mut map = map.clone();
        get_dists_from_end(&mut map, &start);
        map.get(&end).unwrap().1 as isize
    };

    let mut count_above_thresh: usize = 0;

    let possible_cheat_coords: Vec<(isize, isize)> = get_possible_cheat_directions(&map);
    // println!("{:?}", possible_cheat_coords);

    for cheat in possible_cheat_coords {
        let mut map = map.clone();
        map.get_mut(&cheat).unwrap().0 = Object::EmptySpace;
        get_dists_from_end(&mut map, &start);
        let score: isize = map.get(&end).unwrap().1 as isize;
        let savings: isize = best - score;
        // println!(
        //     "Best: {:?}, Score: {:?}, Calculated Savings: {:?}",
        //     best, score, savings
        // );
        if savings >= MIN_GAIN as isize {
            count_above_thresh += 1;
        }
    }
    count_above_thresh
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

fn get_possible_cheat_directions(
    map: &HashMap<(isize, isize), (Object, usize)>,
) -> Vec<(isize, isize)> {
    let mut cheat_vec: Vec<(isize, isize)> = Vec::new();
    for key in map.keys() {
        if let Some(&object) = map.get(key) {
            if object.0 == Object::Wall {
                let up: (isize, isize) = (key.0 - 1, key.1);
                let down: (isize, isize) = (key.0 + 1, key.1);
                let right: (isize, isize) = (key.0, key.1 + 1);
                let left: (isize, isize) = (key.0, key.1 - 1);

                if let Some(&up_obj) = map.get(&up) {
                    if up_obj.0 != Object::Wall {
                        cheat_vec.push(*key);
                        continue;
                    }
                }
                if let Some(&down_obj) = map.get(&down) {
                    if down_obj.0 != Object::Wall {
                        cheat_vec.push(*key);
                        continue;
                    }
                }
                if let Some(&right_obj) = map.get(&right) {
                    if right_obj.0 != Object::Wall {
                        cheat_vec.push(*key);
                        continue;
                    }
                }
                if let Some(&left_obj) = map.get(&left) {
                    if left_obj.0 != Object::Wall {
                        cheat_vec.push(*key);
                        continue;
                    }
                }
            }
        }
    }

    cheat_vec
}
