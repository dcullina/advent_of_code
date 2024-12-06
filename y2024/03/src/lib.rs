use regex::Regex;

pub fn part_one(input: &str) -> i32 {

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|captures| {
            let first_num: i32 = captures[1].parse().unwrap();
            let second_num: i32 = captures[2].parse().unwrap();
            first_num * second_num
        })
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    input.split("do()")
        .filter_map(|subset| {
            Some(subset.split_once("don't()")
                .map(|(before, _)| before)
                .unwrap_or(subset))
        })
        .map(part_one)
        .sum()
}
