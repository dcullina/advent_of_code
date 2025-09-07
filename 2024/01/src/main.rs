fn get_lists(input: &str) -> (Vec<i32>, Vec<i32>) {

    let (mut list_1, mut list_2): (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());

    input.lines()
        .for_each(|line| {
            let numbers: Vec<i32> = line.split_whitespace().
                map(|number| number.parse::<i32>().unwrap())
                .collect();
            list_1.push(numbers[0]);
            list_2.push(numbers[1]);
        });

    (list_1, list_2)
}

fn part_one(input: &str) -> i32 {

    let (mut list_1, mut list_2) = get_lists(input);

    list_1.sort();
    list_2.sort();

    list_1.iter()
        .zip(list_2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part_two(input: &str) -> i32 {

    let (list_1, list_2) = get_lists(input);

    list_1.iter()
        .map(|a| (list_2.iter().filter(|&b| b == a).count() as i32) * a)
        .sum()
}

fn main() {
    let input: &str = include_str!("../input.txt");

    println!("Part One: {:?}", part_one(input));
    println!("Part Two: {:?}", part_two(input));
}
