pub fn part_one(input: &str) -> usize {
    let mut disk: Vec<usize> = input.chars()
        .filter_map(|character| character.to_digit(10))
        .map(|digit| digit as usize)
        .collect();

    let mut checksum: usize = 0;
    let mut flat_index: usize = 0;

    let mut front_index: usize = 0;
    let mut back_index: usize = disk.len() - 1;

    while front_index <= back_index {
        if (front_index % 2) == 1 { // if we are in free space
            // loop through the size of the free space
            for _ in 0..disk[front_index] {
                while disk[back_index] <= 0 {
                    back_index -= 2;
                }
                let id: usize = back_index / 2;
                disk[back_index] -= 1;
                checksum += flat_index * id;
                flat_index += 1;
            }
        } else {
            let id: usize = front_index / 2;
            for _ in 0..disk[front_index] {
                checksum += flat_index * id;
                flat_index += 1;
            }
        }
        front_index += 1;
    }
    checksum
}



pub fn part_two(input: &str) -> usize {
    let block_code: Vec<u8> = input.chars()
        .filter_map(|character| character.to_digit(10))
        .map(|digit| digit as u8)
        .collect();
    1
}
