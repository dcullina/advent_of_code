pub fn part_one(input: &str) -> i32 {
    let matrix: Vec<Vec<char>> = input.lines()
        .map(|line: &str| line.chars().collect())
        .collect();

    let mut total_matches: i32 = 0;

    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, &col_element) in row.iter().enumerate() {
            if col_element == 'X' {
                for row_dir in -1..=1 {
                    for col_dir in -1..=1 {
                        if row_dir == 0 && col_dir == 0 { continue; }
                        if find_next_char(&matrix, row_index, col_index, (row_dir, col_dir), 1) {
                            total_matches += 1;
                        }
                    }
                }
            }
        }
    }
    total_matches
}

fn find_next_char(input_matrix: &Vec<Vec<char>>, row_index: usize, col_index: usize, direction: (i8, i8), distance: u8) -> bool {

    let next_row: i64 = row_index as i64 + direction.0 as i64;
    let next_col: i64 = col_index as i64 + direction.1 as i64;
    
    if next_row < 0 || next_row >= input_matrix.len() as i64 || next_col < 0 || next_col >= input_matrix[0].len() as i64 {
        return false;
    }

    match (distance, input_matrix[next_row as usize][next_col as usize]) {
        (1, 'M') => find_next_char(input_matrix, next_row as usize, next_col as usize, direction, distance+1),
        (2, 'A') => find_next_char(input_matrix, next_row as usize, next_col as usize, direction, distance+1),
        (3, 'S') => true,
        (_, _) => false
    }
}

pub fn part_two(input: &str) -> i32 {
    let matrix: Vec<Vec<char>> = input.lines()
        .map(|line: &str| line.chars().collect())
        .collect();

    let mut total_matches: i32 = 0;

    for (row_index, row) in matrix.iter().enumerate() {
        // skip first and last rows
        if row_index == 0 || row_index == (matrix.len() - 1) {
            continue;
        }
        for (col_index, &col_element) in row.iter().enumerate() {
            // skip first and last columns
            if col_index == 0 || col_index == (matrix[0].len() - 1) {
                continue;
            }
            if col_element == 'A' {
                if find_mas(&matrix, row_index, col_index) {
                    total_matches += 1;
                }
            }
        }
    }
    total_matches
}

fn find_mas(input_matrix: &Vec<Vec<char>>, row_index: usize, col_index: usize) -> bool {
    // Top-Left to Bottom-Right Diagonal
    let diag_tlbr: [char; 2] = [
        input_matrix[row_index - 1][col_index - 1],
        input_matrix[row_index + 1][col_index + 1]
    ];

    // Bottom-Left to Top-Right Diagonal
    let diag_bltr: [char; 2] = [
        input_matrix[row_index + 1][col_index - 1],
        input_matrix[row_index - 1][col_index + 1]
    ];

    let target_sm: [char; 2] = ['S', 'M'];
    let target_ms: [char; 2] = ['M', 'S'];

    (diag_tlbr == target_sm || diag_tlbr == target_ms)
        && (diag_bltr == target_sm || diag_bltr == target_ms)
}
