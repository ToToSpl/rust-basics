use std::fs;

use itertools::Itertools;

const INPUT: &str = "input.txt";

#[derive(Debug, Clone)]
struct Computer {
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    instruction_pointer: usize,
    program: Vec<u8>,
}

impl Computer {
    fn new(input: &str) -> Computer {
        let contents = fs::read_to_string(input).unwrap();
        let mut lines = contents.lines();

        let (reg_a, reg_b, reg_c) = vec!['A', 'B', 'C']
            .into_iter()
            .map(|r| {
                lines
                    .next()
                    .unwrap()
                    .strip_prefix(&format!("Register {r}: "))
                    .unwrap()
                    .parse()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();

        lines.next().unwrap();

        let program = lines
            .next()
            .unwrap()
            .strip_prefix("Program: ")
            .unwrap()
            .split(',')
            .map(|c| c.parse().unwrap())
            .collect();

        Computer {
            reg_a,
            reg_b,
            reg_c,
            instruction_pointer: 0,
            program,
        }
    }

    fn tick(&mut self, output_sink: &mut Vec<u8>) -> Option<()> {
        if self.instruction_pointer >= self.program.len() {
            return None;
        }
        if self.instruction_pointer % 2 == 1 {
            panic!("instruction pointer is not even");
        }

        let opcode = self.program[self.instruction_pointer];
        let operand = self.program[self.instruction_pointer + 1];
        self.instruction_pointer += 2;

        match opcode {
            // adv
            0 => {
                self.reg_a = self.reg_a >> self.combo_operand(operand);
            }
            // bxl
            1 => {
                self.reg_b ^= operand as usize;
            }
            // bst
            2 => {
                self.reg_b = self.combo_operand(operand) & 0b111;
            }
            // jnz
            3 => {
                if self.reg_a != 0 {
                    self.instruction_pointer = operand as usize;
                }
            }
            // bxc
            4 => {
                self.reg_b ^= self.reg_c;
            }
            // out
            5 => {
                output_sink.push((self.combo_operand(operand) & 0b111) as u8);
            }
            // bdv
            6 => {
                self.reg_b = self.reg_a >> self.combo_operand(operand);
            }
            // cdv
            7 => {
                self.reg_c = self.reg_a >> self.combo_operand(operand);
            }
            _o => panic!("wrong opcode: {_o}"),
        }

        Some(())
    }

    fn combo_operand(&self, operand: u8) -> usize {
        match operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _o => panic!("wrong combo operand: {_o}"),
        }
    }
}

fn task1() {
    let mut computer = Computer::new(INPUT);
    let mut sink = Vec::new();

    while computer.tick(&mut sink).is_some() {}

    let score = sink.into_iter().map(|o| o.to_string()).join(",");

    println!("task1:\t{score}");
}

fn check_recur(input: &[usize], i: usize, a: usize) -> Option<usize> {
    let op = input[i];
    let mut a_s = Vec::new();

    for pot_a in (a << 3)..((a << 3) | 0b111) + 1 {
        let a = pot_a;
        let mut b;
        let c;

        b = a & 0b111; // bst 4
        b = b ^ 0b011; // bxl 3
        c = a >> b; // cdv 5
        b = b ^ 0b100; // bxl 4
        b = b ^ c; // bxc 7

        if (b & 0b111) != op {
            continue;
        }
        a_s.push(pot_a);
    }

    if i == 0 {
        a_s.into_iter().min()
    } else {
        a_s.into_iter()
            .filter_map(|a| check_recur(input, i - 1, a))
            .min()
    }
}

fn task2() {
    let input: [usize; 16] = [2, 4, 1, 3, 7, 5, 0, 3, 1, 4, 4, 7, 5, 5, 3, 0];

    let a = check_recur(&input, input.len() - 1, 0).unwrap();

    println!("task2:\t{a:?}");
    println!("input:\t{input:?}");

    let mut computer = Computer::new(INPUT);
    computer.reg_a = a;
    let mut sink = Vec::new();
    while computer.tick(&mut sink).is_some() {}
    let score = sink.into_iter().map(|o| o.to_string()).join(",");

    println!("output:\t{score}");
}

fn main() {
    task1();
    task2();
}
