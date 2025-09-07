use crate::part1::{parse_operation, Operation};
use std::collections::HashMap;

const IO_CHARS: [char; 3] = ['x', 'y', 'z'];

pub fn part_two(input: &str) -> String {
    let mut input_split = input.split("\n\n");

    let _wire_map: HashMap<&str, bool> = input_split
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| {
            line.split_once(": ")
                .map(|(wire, value)| (wire, value == "1"))
        })
        .collect();

    let operations: Vec<(&str, Operation, &str, &str)> = input_split
        .next()
        .unwrap()
        .lines()
        .map(parse_operation)
        .collect();

    let mut incorrect_wires: Vec<&str> = Vec::new();
    for operation in &operations {
        if operation.3.starts_with('z') && operation.1 != Operation::Xor && operation.3 != "z45" {
            incorrect_wires.push(operation.3);
        }

        if operation.1 == Operation::Xor
            && !IO_CHARS.contains(&operation.0.chars().next().unwrap())
            && !IO_CHARS.contains(&operation.2.chars().next().unwrap())
            && !IO_CHARS.contains(&operation.3.chars().next().unwrap())
        {
            incorrect_wires.push(operation.3);
        }

        if operation.1 == Operation::And && ![operation.0, operation.2].contains(&"x00") {
            for sub_operation in &operations {
                if (operation.3 == sub_operation.0 || operation.3 == sub_operation.2)
                    && sub_operation.1 != Operation::Or
                {
                    incorrect_wires.push(operation.3);
                }
            }
        }

        if operation.1 == Operation::Xor {
            for sub_operation in &operations {
                if (operation.3 == sub_operation.0 || operation.3 == sub_operation.2)
                    && sub_operation.1 == Operation::Or
                {
                    incorrect_wires.push(operation.3);
                }
            }
        }
    }

    incorrect_wires.sort();
    incorrect_wires.dedup();
    incorrect_wires.join(",")
}
