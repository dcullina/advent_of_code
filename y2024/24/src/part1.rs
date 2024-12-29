use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Xor,
    Or,
    And,
}

pub fn part_one(input: &str) -> usize {
    let mut input_split = input.split("\n\n");

    let mut wire_map: HashMap<&str, bool> = HashMap::new();
    input_split.next().unwrap().lines().for_each(|line| {
        if let Some((wire, value)) = line.split_once(": ") {
            // println!("wire: {:?}, value: {:?}", wire, value);
            let wire_value: bool = match value {
                "1" => true,
                "0" => false,
                _ => unreachable!(),
            };
            wire_map.insert(wire, wire_value);
        }
    });
    let mut operations: Vec<&str> = input_split.next().unwrap().lines().collect();

    let mut index: usize = 0;
    while !operations.is_empty() {
        // println!("{:?}", operations.len());

        if index == operations.len() {
            index = 0;
        }

        let (wire_a, operator, wire_b, wire_c) = parse_operation(operations[index]);
        // println!("{:?}, {:?}, {:?}, {:?}", wire_a, operator, wire_b, wire_c);
        if !wire_map.contains_key(wire_a) || !wire_map.contains_key(wire_b) {
            index += 1;
            continue;
        }
        match operator {
            Operation::Xor => wire_map.insert(wire_c, wire_map[wire_a] ^ wire_map[wire_b]),
            Operation::Or => wire_map.insert(wire_c, wire_map[wire_a] | wire_map[wire_b]),
            Operation::And => wire_map.insert(wire_c, wire_map[wire_a] & wire_map[wire_b]),
        };

        operations.remove(index);
    }
    compute_result(&wire_map)
}

pub fn compute_result(wire_map: &HashMap<&str, bool>) -> usize {
    let mut result: usize = 0;
    for wire in wire_map.keys() {
        if wire.starts_with('z') {
            let bit_position: usize = wire.strip_prefix('z').unwrap().parse::<usize>().unwrap();
            result |= (wire_map[wire] as usize) << bit_position;
        }
    }
    result
}

pub fn parse_operation(operation: &str) -> (&str, Operation, &str, &str) {
    let parts: Vec<&str> = operation.split_whitespace().collect();

    (
        parts[0],
        match parts[1] {
            "XOR" => Operation::Xor,
            "OR" => Operation::Or,
            "AND" => Operation::And,
            _ => unreachable!(),
        },
        parts[2],
        parts[4],
    )
}
