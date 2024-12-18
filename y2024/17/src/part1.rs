pub struct Machine3B<'a> {
    pub register_a: usize,
    register_b: usize,
    register_c: usize,
    instruction_pointer: usize,
    pub instruction_set: &'a Vec<usize>,
}

impl Machine3B<'_> {
    pub fn new(register_a: usize, instruction_set: &Vec<usize>) -> Machine3B<'_> {
        Machine3B {
            register_a,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            instruction_set,
        }
    }

    pub fn reset_machine(&mut self) {
        self.register_b = 0;
        self.register_c = 0;
        self.instruction_pointer = 0;
    }

    pub fn run(&mut self) -> Option<usize> {
        while self.instruction_pointer < self.instruction_set.len() {
            let opcode: usize = self.instruction_set[self.instruction_pointer];
            let literal_operand: usize = self.instruction_set[self.instruction_pointer + 1];

            let mut combo_operand: usize = 0;
            match literal_operand {
                operand @ 0..4 => combo_operand = operand,
                4 => combo_operand = self.register_a,
                5 => combo_operand = self.register_b,
                6 => combo_operand = self.register_c,
                _ => (),
            }

            match opcode {
                0 => {
                    // adv
                    self.register_a >>= combo_operand;
                }
                1 => {
                    // bxl
                    self.register_b ^= literal_operand;
                }
                2 => {
                    // bst
                    self.register_b = combo_operand % 8;
                }
                3 => {
                    // jnz
                    if self.register_a != 0 {
                        self.instruction_pointer = literal_operand;
                        continue;
                    }
                }
                4 => {
                    // bxc
                    self.register_b ^= self.register_c;
                }
                5 => {
                    // out
                    self.instruction_pointer += 2;
                    return Some(combo_operand % 8);
                }
                6 => {
                    // bdv
                    self.register_b = self.register_a >> combo_operand;
                }
                7 => {
                    // cdv
                    self.register_c = self.register_a >> combo_operand;
                }
                _ => (),
            }

            self.instruction_pointer += 2;
        }

        None
    }
}

pub fn part_one(input: &str) -> String {
    let mut split_input = input.split("\n\n");

    let init_reg_a_val: usize = split_input
        .next()
        .unwrap()
        .lines()
        .find(|line| line.starts_with("Register A:"))
        .and_then(|line| line.split(": ").nth(1))
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let instructions: Vec<usize> = split_input
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(',')
        .map(|value| value.replace('\n', "").parse::<usize>().unwrap())
        .collect();

    let mut machine: Machine3B = Machine3B::new(init_reg_a_val, &instructions);
    let mut output: Vec<u8> = Vec::new();
    while let Some(number) = machine.run() {
        output.push(number as u8);
    }
    output
        .iter()
        .map(|number| number.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
