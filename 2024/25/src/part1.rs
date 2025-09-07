use std::collections::HashSet;
const GRID_WIDTH: usize = 5;
const GRID_HEIGHT: usize = 7;

#[derive(Debug, PartialEq)]
enum MapType {
    Lock,
    Key,
}

pub fn part_one(input: &str) -> usize {
    let maps: Vec<(MapType, [u8; GRID_WIDTH])> = input.split("\n\n").map(parse_map).collect();
    let pairs: HashSet<(usize, usize)> = find_pairs(&maps);
    pairs.len()
}

fn find_pairs(maps: &Vec<(MapType, [u8; GRID_WIDTH])>) -> HashSet<(usize, usize)> {
    let mut pairs: HashSet<(usize, usize)> = HashSet::new();
    for (key_index, key) in maps
        .iter()
        .enumerate()
        .filter(|(_, map)| map.0 == MapType::Key)
    {
        for (lock_index, lock) in maps
            .iter()
            .enumerate()
            .filter(|(_, map)| map.0 == MapType::Lock)
        {
            if (0..GRID_WIDTH)
                .any(|column| (key.1[column] + lock.1[column]) > ((GRID_HEIGHT - 2) as u8))
            {
                continue;
            }
            pairs.insert((key_index, lock_index));
        }
    }
    pairs
}

fn parse_map(map: &str) -> (MapType, [u8; GRID_WIDTH]) {
    let map_type: MapType = match map.starts_with('#') {
        true => MapType::Lock,
        false => MapType::Key,
    };

    let mut values: [u8; GRID_WIDTH] = [0; GRID_WIDTH];
    if map_type == MapType::Lock {
        for (row_index, line) in map.lines().skip(1).enumerate() {
            for (column_index, column_val) in line.chars().enumerate() {
                if column_val == '#' {
                    values[column_index] = row_index as u8 + 1;
                }
            }
        }
    } else {
        for (row_index, line) in map.lines().rev().skip(1).enumerate() {
            for (column_index, column_val) in line.chars().enumerate() {
                if column_val == '#' {
                    values[column_index] = row_index as u8 + 1;
                }
            }
        }
    }

    (map_type, values)
}
