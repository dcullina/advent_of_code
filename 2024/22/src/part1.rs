const MODULO_BY: usize = 16777216;

pub fn part_one<const NUM_NEW_SECRET_NUMBERS: usize>(input: &str) -> usize {
    let buyers: Vec<usize> = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    buyers
        .iter()
        .map(|&buyer| {
            // print!("Starting: {:?}, ", buyer);
            let mut value = buyer;
            for _ in 0..NUM_NEW_SECRET_NUMBERS {
                value = compute_next_secret(value);
            }
            // println!("After: {:?}", value);
            value
        })
        .sum()
}

pub fn compute_next_secret(mut secret_number: usize) -> usize {
    secret_number = (secret_number ^ (secret_number << 6)).rem_euclid(MODULO_BY);
    secret_number = (secret_number ^ (secret_number >> 5)).rem_euclid(MODULO_BY);
    (secret_number ^ (secret_number << 11)).rem_euclid(MODULO_BY)
}
