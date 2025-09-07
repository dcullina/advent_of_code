use crate::part1::Machine3B;
use std::ops::ControlFlow;

pub fn part_two(input: &str) -> usize {
    
    let instructions: Vec<usize> = input.split("\n\n").nth(1).unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(',')
        .map(|value| {
            value.replace('\n', "").parse::<usize>().unwrap()
        })
        .collect();

    solve(&instructions, (instructions.len() - 1) as isize, 0).break_value().unwrap()
}

fn solve(instructions: &Vec<usize>, index: isize, reg_a_initializer: usize) -> ControlFlow<usize> {
    if index == -1 {
        return ControlFlow::Break(reg_a_initializer);
    }

    for i in 0..8 {
        let next_a_initializer = (reg_a_initializer << 3) | i;
        let output = Machine3B::new(next_a_initializer, instructions).run().unwrap();

        if output == instructions[index as usize] {
            solve(instructions, index - 1, next_a_initializer)?;
        }
    }
    ControlFlow::Continue(())
}