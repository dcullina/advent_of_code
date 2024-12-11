const MUL_YEAR: usize = 2024;

pub fn part_one(input: &str, n_blinks: usize) -> usize {
    let mut stones: Vec<usize> = Vec::new();
    input
        .split_ascii_whitespace()
        .for_each(|number| {
            stones.push(number.parse::<usize>().unwrap());
        });

    for _ in 0..n_blinks {
        let mut index: usize = 0;
        loop {
            if index >= stones.len() { break; }
            if stones[index] == 0 {
                stones[index] = 1;
            } else {
                let val_str = stones[index].to_string();
                if val_str.chars().count() % 2 == 0 {

                    let (left, right) = val_str.split_at(val_str.len() / 2);

                    stones[index] = left.parse::<usize>().unwrap();

                    if stones.len() == index + 1 {
                        stones.push(right.parse::<usize>().unwrap());
                    } else {
                        stones.insert(index + 1, right.parse::<usize>().unwrap())
                    }

                    index += 1;
                } else {
                    stones[index] *= MUL_YEAR;
                }
            }
            index += 1;
        }
    }
    stones.len()
}


pub fn part_two(input: &str, n_blinks: usize) -> usize {
    1
}