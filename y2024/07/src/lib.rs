pub fn part_one(input: &str) -> u64 {
    let input_set: Vec<(u64, Vec<u64>)> = input.replace(":","")
        .lines()
        .map(|line: &str| {
            let row: Vec<u64> = line.split_whitespace()
                .filter_map(|number: &str| number.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            (row[0], row[1..].to_vec())
        })
        .collect::<Vec<(u64, Vec<u64>)>>();


    input_set.iter()
        .map(|line: &(u64, Vec<u64>)| {
            if check_valid(&line.0, &line.1) {
                line.0
            } else {
                0
            }
        }).sum::<u64>()
}

fn check_valid(target: &u64, numbers: &Vec<u64>) -> bool {
    let total_combinations: usize = 1 << (numbers.len() - 1);

    for combination in 0..total_combinations {

        let mut calculated_value = numbers[0];
        let mut result: bool = true;

        for (index, &number) in numbers[1..].iter().enumerate() {
            if ((combination >> index) & 1) == 1 {
                calculated_value *= number;
            } else {
                calculated_value += number;
            }

            if calculated_value > *target {
                result = false;
                break;
            }

        }

        if result && (calculated_value == *target) {
            return true;
        }

    }
    false
}

pub fn part_two(input: &str) -> u64 {
    1
}
