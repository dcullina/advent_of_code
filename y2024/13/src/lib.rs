const PART_TWO_OFFSET: usize = 10000000000000;

pub fn part_one(input: &str) -> usize {
    let machines: Vec<Vec<(usize,usize)>> = input
        .split("\n\n")
        .map(|machine| {
            machine
                .lines()
                .enumerate()
                .map(|(index, command)| {
                    if index == 2 {
                        let split_command: Vec<&str> = command.split(&['=', ','][..]).collect();
                        (split_command[1].parse::<usize>().unwrap(), split_command[3].parse::<usize>().unwrap())
                    } else {
                        let split_command: Vec<&str> = command.split(&['+', ','][..]).collect();
                        (split_command[1].parse::<usize>().unwrap(), split_command[3].parse::<usize>().unwrap())
                    }
                })
                .collect()
        })
        .collect();

    machines.iter()
        .map(|machine| {
            calculate_cost(machine)
        })
        .sum()
}

fn calculate_cost(machine: &Vec<(usize, usize)>) -> usize {
    let A: (isize, isize) = (machine[0].0 as isize, machine[0].1 as isize);
    let B: (isize, isize) = (machine[1].0 as isize, machine[1].1 as isize);
    let P: (isize, isize) = (machine[2].0 as isize, machine[2].1 as isize);

    let a_det: isize = P.0 * B.1 - P.1 * B.0;
    let b_det: isize = A.0 * P.1 - A.1 * P.0;
    let det: isize = A.0 * B.1 - A.1 * B.0;

    let a: Option<isize> = check_division(a_det, det);
    let b: Option<isize> = check_division(b_det, det);
    if a == None || b == None {
        return 0;
    }

    (a.unwrap() * 3 + b.unwrap()) as usize
}

fn check_division(numerator: isize, denominator: isize) -> Option<isize> {
    if denominator == 0 {
        None
    } else if numerator % denominator != 0 {
        None
    } else {
        Some( numerator / denominator )
    }
}

pub fn part_two(input: &str) -> usize {
    let machines: Vec<Vec<(usize,usize)>> = input
        .split("\n\n")
        .map(|machine| {
            machine
                .lines()
                .enumerate()
                .map(|(index, command)| {
                    if index == 2 {
                        let split_command: Vec<&str> = command.split(&['=', ','][..]).collect();
                        (
                            split_command[1].parse::<usize>().unwrap() + PART_TWO_OFFSET,
                            split_command[3].parse::<usize>().unwrap() + PART_TWO_OFFSET
                        )
                    } else {
                        let split_command: Vec<&str> = command.split(&['+', ','][..]).collect();
                        (
                            split_command[1].parse::<usize>().unwrap(),
                            split_command[3].parse::<usize>().unwrap()
                        )
                    }
                })
                .collect()
        })
        .collect();

    machines.iter()
        .map(|machine| {
            calculate_cost(machine)
        })
        .sum()
}
