use std::{collections::{HashMap, HashSet}, num::ParseFloatError};

pub fn part_one(input: &str) -> usize {
    let mut map: Vec<Vec<(char, bool)>> = input
        .lines()
        .enumerate()
        .map(|(row_index, line)| {
            line
                .chars()
                .map(|character| (character, false))
                .collect()
        })
        .collect();
    let mut sum: usize = 0;

    for row_index in 0..map.len() {
        for col_index in 0..map[0].len() {
            // skip it if we already got to it
            if map[row_index][col_index].1 { continue; }
            let target_char: char = map[row_index][col_index].0;
            let mut dimensions: (usize, usize) = (0, 0);
            dfs_explore_p1(
                &mut map,
                target_char,
                row_index,
                col_index,
                &mut dimensions // (perimeter count, area count)
            );

            sum += dimensions.0 * dimensions.1;
        }
    }

    sum
}

fn dfs_explore_p1(map: &mut Vec<Vec<(char, bool)>>, target_char: char, row_index: usize, col_index: usize, dimensions: &mut (usize, usize)) {

    // exit if we have already seen this
    if map[row_index][col_index].1 { return; }

    // set it as seen
    map[row_index][col_index].1 = true;

    // add it to the area
    dimensions.1 += 1;

    // Check Up - decreasing row
    if let Some(new_row_index) = row_index.checked_sub(1) {
        if map[new_row_index][col_index].0 == target_char {
            dfs_explore_p1(
                map,
                target_char,
                new_row_index,
                col_index,
                dimensions
            );
        } else {
            dimensions.0 += 1;
        }
    } else {
        dimensions.0 += 1;
    }

    // Check Down - increasing row
    if let Some(row) = map.get(row_index + 1) {
        if row[col_index].0 ==  target_char {
            dfs_explore_p1(
                map,
                target_char,
                row_index + 1,
                col_index,
                dimensions
            );
        } else {
            dimensions.0 += 1;
        }
    } else {
        dimensions.0 += 1;
    }

    // Check Left - decreasing column
    if let Some(new_col_index) = col_index.checked_sub(1) {
        if map[row_index][new_col_index].0 == target_char {
            dfs_explore_p1(
                map,
                target_char,
                row_index,
                new_col_index,
                dimensions
            );
        } else {
            dimensions.0 += 1;
        }
    } else {
        dimensions.0 += 1;
    }

    // Check Right - increasing column
    if let Some(element) = map[row_index].get(col_index + 1) {
        if element.0 == target_char {
            dfs_explore_p1(
                map,
                target_char,
                row_index,
                col_index + 1,
                dimensions
            );
        } else {
            dimensions.0 += 1;
        }
    } else {
        dimensions.0 += 1;
    }
}










const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0)
];

const CORNER_TYPES: [[(isize, isize); 3]; 4] = [
    [
        // top left
        (-1, -1),
        (-1, 0),
        (0, -1)
    ],
    [
        // bottom left
        (1, -1),
        (1, 0),
        (0, -1)
    ],
    [
        // bottom right
        (1, 1),
        (1, 0),
        (0, 1)
    ],
    [
        // top right
        (-1, 1),
        (-1, 0),
        (0, 1)
    ]
];

pub fn part_two(input: &str) -> usize {
    let mut map: HashMap<(isize, isize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(row_index, line)| {
            line
                .chars()
                .enumerate()
                .map(move |(col_index, character)| ((row_index as isize, col_index as isize), character))
        })
        .collect();

    let mut regions: Vec<HashSet<(isize, isize)>> = Vec::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    for location in map.keys() {
        if visited.contains(location) { continue; }
        let mut region: HashSet<(isize, isize)> = HashSet::new();
        explore_region(&map, location, map[location], &mut region);
        visited.extend(&region);
        regions.push(region);
    }

    regions
        .iter()
        .map(|region| {
            region
                .iter()
                .map(|location| check_corners(&map, location))
                .sum::<usize>() * region.len()
        })
        .sum::<usize>()

}


fn explore_region(
    map: &HashMap<(isize, isize), char>,
    location: &(isize, isize),
    target_char: char,
    region: &mut HashSet<(isize, isize)>

) {
    region.insert(*location);
    for next_location in explore_directions(map, location) {
        if map[&next_location] == target_char && !region.contains(&next_location) {
            explore_region(map, &next_location, target_char, region);
        }
    }
}

fn explore_directions(map: &HashMap<(isize, isize), char>, location: &(isize, isize)) -> Vec<(isize, isize)> {
    DIRECTIONS
        .iter()
        .map(|direction| {
            (
                location.0 + direction.0,
                location.1 + direction.1
            )
        })
        .filter(|next_location| {
            map.contains_key(next_location)
        })
        .collect()
}

fn check_corners(map: &HashMap<(isize, isize), char>, location: &(isize, isize)) -> usize {
    CORNER_TYPES
        .iter()
        .filter(|&corner| {
            let diagonal = map.get(&(
                location.0 + corner[0].0,
                location.1 + corner[0].1
            ));
            let lateral_column = map.get(&(
                location.0 + corner[1].0,
                location.1 + corner[1].1
            ));
            let lateral_row = map.get(&(
                location.0 + corner[2].0,
                location.1 + corner[2].1
            ));
            let character= map.get(location);
            (character != lateral_row && character != lateral_column)
                || (character == lateral_row && character == lateral_column && character != diagonal)
        })
        .count()
}