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
    let disk: Vec<usize> = input.chars()
        .filter_map(|character| character.to_digit(10))
        .map(|digit| digit as usize)
        .collect();

    // convert it to a vector of (file_id/EmptySpace, size)
    let mut file_rep: Vec<(Option<usize>, usize, bool)> = disk
        .iter()
        .enumerate()
        .map(|(index, &size)| {
            if (index % 2) == 1 {
                (None, size, false)
            } else {
                (Some(index / 2), size, false)
            }
        }).collect();

    let mut back_index: usize = file_rep.len() - 1; // set at the last file

    'outer: while back_index > 0 {
        // if we visited this or if it's empty space skip it
        if file_rep[back_index].2 || file_rep[back_index].0.is_none() {
            back_index -= 1;
            continue;
        }

        for index in 0..back_index {
            match file_rep[index].0 {
                None => {
                    // if it fits in the blank space, insert it, remove it, then break
                    if file_rep[index].1 >= file_rep[back_index].1 {
                        // insert that element into the space
                        file_rep.insert(index, (
                            file_rep[back_index].0,
                            file_rep[back_index].1,
                            true,
                        ));
                        // subtract the size of the white space
                        file_rep[index + 1].1 -= file_rep[index].1;

                        // change that element to empty space
                        file_rep[back_index + 1].0 = None;
                        continue 'outer; // this will restart the outer loop with back index pointed at the next variable now
                    }
                },
                _ => continue
            }
        }
        // if there are no matches, bring the back index down by one
        back_index -= 1;
    }


    let mut checksum: usize = 0; // used to collect id * index sum for each block
    let mut flat_index: usize = 0; // used for tracking the index to calculate checksum

    file_rep.iter().for_each(|element| {
        if let Some(file_id) = element.0 {
            for _ in 0..element.1 {
                checksum += file_id * flat_index;
                flat_index += 1;
            }
        } else {
            flat_index += element.1;
        }
    });

    checksum
}
