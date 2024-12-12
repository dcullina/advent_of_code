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
            dfs_explore(
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

fn dfs_explore(map: &mut Vec<Vec<(char, bool)>>, target_char: char, row_index: usize, col_index: usize, dimensions: &mut (usize, usize)) {

    // exit if we have already seen this
    if map[row_index][col_index].1 { return; }

    // set it as seen
    map[row_index][col_index].1 = true;

    // add it to the area
    dimensions.1 += 1;

    // Check Up - decreasing row
    if let Some(new_row_index) = row_index.checked_sub(1) {
        if map[new_row_index][col_index].0 == target_char {
            dfs_explore(
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
            dfs_explore(
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
            dfs_explore(
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
            dfs_explore(
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

pub fn part_two(input: &str) -> usize {

    1
}