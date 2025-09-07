use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line: &str| 
            line.chars().collect()
        )
        .collect();

    let mut antenna_map: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

    grid.iter().enumerate()
        .for_each(|(row_index, row)| {
            row.iter().enumerate()
                .for_each(|(col_index, &element)| {
                    if element.is_ascii_alphanumeric() {
                        antenna_map
                            .entry(element)
                            .or_insert_with(HashSet::new)
                            .insert((row_index, col_index));
                    }
                });
        });

    let mut unique_locations: HashSet<(usize, usize)> = HashSet::new();

    for (_, coords) in antenna_map {
        let coord_vec: Vec<_> = coords.iter().collect();
        for i in 0..coord_vec.len() {
            for j in (i + 1)..coord_vec.len() {
                let coord_diff: (isize, isize) = (
                    coord_vec[i].0 as isize - coord_vec[j].0 as isize,
                    coord_vec[i].1 as isize - coord_vec[j].1 as isize
                );

                let new_i: (isize, isize) = (
                    coord_vec[i].0 as isize + coord_diff.0,
                    coord_vec[i].1 as isize + coord_diff.1,
                );

                if (new_i.0 >= 0) && (new_i.0 < grid.len() as isize)
                    && (new_i.1 >= 0) && (new_i.1 < grid[0].len() as isize) {
                        unique_locations.insert((new_i.0 as usize, new_i.1 as usize));
                    }

                let new_j: (isize, isize) = (
                    coord_vec[j].0 as isize - coord_diff.0,
                    coord_vec[j].1 as isize - coord_diff.1,
                );

                if (new_j.0 >= 0) && (new_j.0 < grid.len() as isize)
                    && (new_j.1 >= 0) && (new_j.1 < grid[0].len() as isize) {
                        unique_locations.insert((new_j.0 as usize, new_j.1 as usize));
                    }
            }
        }
    }
    unique_locations.len() as u64
}

pub fn part_two(input: &str) -> u64 {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line: &str| 
            line.chars().collect()
        )
        .collect();

    let mut antenna_map: HashMap<char, HashSet<(usize, usize)>> = HashMap::new();

    grid.iter().enumerate()
        .for_each(|(row_index, row)| {
            row.iter().enumerate()
                .for_each(|(col_index, &element)| {
                    if element.is_ascii_alphanumeric() {
                        antenna_map
                            .entry(element)
                            .or_insert_with(HashSet::new)
                            .insert((row_index, col_index));
                    }
                });
        });

    let mut unique_locations: HashSet<(usize, usize)> = HashSet::new();

    for (_, coords) in antenna_map {
        let coord_vec: Vec<_> = coords.iter().collect();
        for i in 0..coord_vec.len() {
            for j in (i + 1)..coord_vec.len() {
                
                unique_locations.insert(*coord_vec[i]);
                unique_locations.insert(*coord_vec[j]);

                let mut a: (isize, isize) = (coord_vec[i].0 as isize, coord_vec[i].1 as isize);
                let mut b: (isize, isize) = (coord_vec[j].0 as isize, coord_vec[j].1 as isize);

                let coord_diff: (isize, isize) = (
                    a.0 as isize - b.0 as isize,
                    a.1 as isize - b.1 as isize
                );

                loop {

                    a = (
                        a.0 + coord_diff.0,
                        a.1 + coord_diff.1,
                    );

                    if (a.0 >= 0) && (a.0 < grid.len() as isize)
                        && (a.1 >= 0) && (a.1 < grid[0].len() as isize) {
                            unique_locations.insert((a.0 as usize, a.1 as usize));
                    } else {
                        break;
                    }

                }

                loop {

                    b = (
                        b.0 - coord_diff.0,
                        b.1 - coord_diff.1,
                    );

                    if (b.0 >= 0) && (b.0 < grid.len() as isize)
                        && (b.1 >= 0) && (b.1 < grid[0].len() as isize) {
                            unique_locations.insert((b.0 as usize, b.1 as usize));
                    } else {
                        break;
                    }
                }
            }
        }
    }
    unique_locations.len() as u64
}
