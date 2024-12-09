pub fn part_one(input: &str) -> usize {
    let block_code: Vec<u8> = input.chars()
        .filter_map(|character| character.to_digit(10))
        .map(|digit| digit as u8)
        .collect();
    let id_code: Vec<i32> = convert_to_id_code(block_code);
    calculate_checksum(id_code)
}

fn convert_to_id_code(block_code: Vec<u8>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut id: u32 = 0;
    for (index, &digit) in block_code.iter().enumerate() {
        if (index % 2) == 0 {
            for _ in 0..digit {
                result.push(id as i32);
            }
            id += 1;
        } else {
            for _ in 0..digit {
                result.push(-1);
            }
        }
    }
    result
}

fn calculate_checksum(mut id_code: Vec<i32>) -> usize {
    let mut sum: usize = 0;
    let mut index: usize = 0;
    while !id_code.is_empty() {
        if index >= id_code.len() { break; }
        if id_code[id_code.len() - 1] == -1 { 
            id_code.pop();
            continue;
        }
        if id_code[index] == -1 {
            sum += index * id_code.pop().unwrap_or(0) as usize;
        } else {
            sum += index * id_code[index] as usize;
        }
        index += 1;
    }
    sum
}

pub fn part_two(input: &str) -> usize {
    1
}
