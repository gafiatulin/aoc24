use itertools::Itertools;

#[derive(Clone)]
pub struct Computer {
    register_a: i128,
    register_b: i128,
    register_c: i128,
    instruction_pointer: u8,
    program: Vec<i128>,
    output: Vec<i128>,
}

impl Computer {
    fn halted(&self) -> bool {
        self.instruction_pointer as usize >= self.program.len()
    }

    fn run(&mut self) {
        while !self.halted() {
            self.execute();
        }
    }

    fn combo_operand_value(&self) -> i128 {
        match self.program[self.instruction_pointer as usize + 1] {
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            x => x,
        }
    }

    fn execute(&mut self) {
        match self.program[self.instruction_pointer as usize] {
            0 => {
                let co = self.combo_operand_value();
                self.register_a = division(self.register_a, co);
                self.instruction_pointer += 2;
            }
            1 => {
                let lo = self.program[self.instruction_pointer as usize + 1];
                self.register_b ^= lo;
                self.instruction_pointer += 2;
            }
            2 => {
                let co = self.combo_operand_value();
                self.register_b = co % 8;
                self.instruction_pointer += 2;
            }
            3 => {
                if self.register_a != 0 {
                    self.instruction_pointer =
                        self.program[self.instruction_pointer as usize + 1] as u8;
                } else {
                    self.instruction_pointer += 2;
                }
            }
            4 => {
                self.register_b ^= self.register_c;
                self.instruction_pointer += 2;
            }
            5 => {
                let co = self.combo_operand_value();
                self.output.push(co % 8);
                self.instruction_pointer += 2;
            }
            6 => {
                let co = self.combo_operand_value();
                self.register_b = division(self.register_a, co);
                self.instruction_pointer += 2;
            }
            7 => {
                let co = self.combo_operand_value();
                self.register_c = division(self.register_a, co);
                self.instruction_pointer += 2;
            }
            _ => {}
        }
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Computer {
    let register_a = parse_register("A", input);
    let register_b = parse_register("B", input);
    let register_c = parse_register("C", input);
    let program = parse_program(input);

    Computer {
        register_a,
        register_b,
        register_c,
        instruction_pointer: 0,
        program,
        output: Vec::new(),
    }
}

#[aoc(day17, part1)]
pub fn part1(computer: &Computer) -> String {
    let mut computer = computer.clone();

    computer.run();

    computer.output.iter().join(",")
}

#[aoc(day17, part2)]
pub fn part2(computer: &Computer) -> i128 {
    part2_solver(computer, computer.program.len() - 1, 0).unwrap()
}

fn parse_register(label: &str, input: &str) -> i128 {
    input
        .lines()
        .filter(|line| line.starts_with(format!("Register {}:", label).as_str()))
        .map(|line| line.split_once(":").unwrap().1.trim().parse().unwrap())
        .next()
        .unwrap()
}

fn parse_program(input: &str) -> Vec<i128> {
    input
        .lines()
        .filter(|line| line.starts_with("Program:"))
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .trim()
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<i128>>()
        })
        .next()
        .unwrap()
}

fn division(numerator: i128, denominator: i128) -> i128 {
    let result = (numerator as f64) / (2_i128.pow(denominator as u32) as f64);
    result as i128
}

fn part2_solver(computer: &Computer, idx: usize, acc: i128) -> Option<i128> {
    let c = computer.clone();

    (0..8).find_map(|o| {
        let check = acc * 8 + o;
        let mut curr = c.clone();
        curr.register_a = check;
        curr.run();
        if curr.output == c.program[idx..].to_vec() {
            if idx == 0 {
                Some(check)
            } else {
                part2_solver(&c, idx - 1, check)
            }
        } else {
            None
        }
    })
}
