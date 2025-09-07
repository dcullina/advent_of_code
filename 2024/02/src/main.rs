fn part_one(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect();

            let ascending: bool = numbers[0] < numbers[1];

            !numbers.windows(2).any(|pair| {
                let difference: i32 = (pair[0] - pair[1]).abs();
                let breaking_allowed_change: bool = (difference > 3) || (difference < 1);
                let breaking_order: bool =
                (ascending && (pair[0] > pair[1])) || (!ascending && (pair[0] < pair[1]));

                breaking_allowed_change || breaking_order
            })
        })
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|value| value.parse::<i32>().unwrap())
                .collect();

            let mut safe: bool = false;

            // check if it's safe for at least one subset
            for index in 0..numbers.len() {
                let mut new_numbers: Vec<i32> = numbers.clone();
                new_numbers.remove(index);

                let ascending: bool = new_numbers[0] < new_numbers[1];

                let number_of_bad_pairs: usize = new_numbers
                    .windows(2)
                    .filter(|pair| {
                        let difference: i32 = (pair[0] - pair[1]).abs();
                        let breaking_allowed_change: bool = (difference > 3) || (difference < 1);
                        let breaking_order: bool =
                        (ascending && (pair[0] > pair[1])) || (!ascending && (pair[0] < pair[1]));

                        breaking_allowed_change || breaking_order
                    })
                    .count();

                if number_of_bad_pairs == 0 {
                    safe = true;
                    break;
                }
            }

            safe

        })
        .count()
}

#[test]

fn main() {
    //let input = include_str!("../test_input.txt");
    let input = include_str!("../input.txt");

    println!("Part One: {:?}", part_one(input));
    println!("Part Two: {:?}", part_two(input));
}
